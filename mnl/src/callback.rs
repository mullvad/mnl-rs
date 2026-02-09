use crate::NlMessages;
use mnl_sys::{self, libc};

use std::{io, ptr};

/// The result of processing a batch of netlink responses.
pub enum CbResult {
    /// Everything went fine and this batch is finished processing.
    Stop,
    /// Everything went fine, but we expect more messages to come back from the kernel for this
    /// batch.
    Ok,
}

/// Callback function signature.
/// TODO: Write abstraction for `nlmsghdr` that can reach all fields and payload.
pub type Callback<T> = fn(msg: &libc::nlmsghdr, data: &mut T) -> libc::c_int;

/// Callback runqueue for netlink messages. Checks that all netlink messages in `buffer` are OK.
/// `buffer` must be aligned to `size_of::<nlmsghdr>()`, or this fails.
pub fn cb_run(buffer: &[u8], seq: u32, portid: u32) -> io::Result<CbResult> {
    // NOTE: See comment on [`validate_messages`] for why we need to validate messages here.
    validate_messages(buffer)?;

    let len = buffer.len();
    let buf = buffer.as_ptr() as *const libc::c_void;

    log::debug!("Processing {} byte netlink message without a callback", len);
    match unsafe { mnl_sys::mnl_cb_run(buf, len, seq, portid, None, ptr::null_mut()) } {
        i if i <= mnl_sys::MNL_CB_ERROR => Err(io::Error::last_os_error()),
        mnl_sys::MNL_CB_STOP => Ok(CbResult::Stop),
        _ => Ok(CbResult::Ok),
    }
}

/// Callback runqueue for netlink messages. Checks that all netlink messages in `buffer` are OK.
/// Calls the given `callback` if needed.
/// `buffer` must be aligned to `size_of::<nlmsghdr>()`, or this fails.
pub fn cb_run2<T>(
    buffer: &[u8],
    seq: u32,
    portid: u32,
    callback: Callback<T>,
    data: &mut T,
) -> io::Result<CbResult> {
    // NOTE: See comment on [`validate_messages`] for why we need to validate messages here.
    validate_messages(buffer)?;

    let len = buffer.len();
    let buf = buffer.as_ptr() as *const libc::c_void;
    let mut callback_context = CallbackContext { callback, data };
    log::debug!("Processing {} byte netlink message with callback", len);
    match unsafe {
        mnl_sys::mnl_cb_run(
            buf,
            len,
            seq,
            portid,
            Some(callback_wrapper::<T>),
            &mut callback_context as *mut _ as *mut libc::c_void,
        )
    } {
        i if i <= mnl_sys::MNL_CB_ERROR => Err(io::Error::last_os_error()),
        mnl_sys::MNL_CB_STOP => Ok(CbResult::Stop),
        _ => Ok(CbResult::Ok),
    }
}

/// libmnl contains a bug in `mnl_nlmsg_ok` where it casts `nlh->nlmsg_len` to an `int`,
/// i.e. `(int)nlh->nlmsg_len`. This becomes negative if `nlmsg_len` is greater than `INT_MAX`,
/// causing the validation to succeed even if the buffer is too small. `mnl_nlmsg_ok` is
/// used by `mnl_cb_run` and `mnl_cb_run2`.
///
/// This was fixed on 2023-11-04 in commit `754c9de5ea1bea821495523cf01989299552e524`,
/// but the latest version of libmnl 1.0.5 was released on 2022-04-05, so as of writing
/// there is no released version of libmnl that contains the fix.
///
/// Thus we need our own validation.
///
/// See the libmnl git repo and that commit for details: git://git.netfilter.org/libmnl
///
/// This addresses [RUSTSEC-2025-0142](https://rustsec.org/advisories/RUSTSEC-2025-0142.html).
fn validate_messages(buffer: &[u8]) -> io::Result<()> {
    NlMessages::new(buffer).try_for_each(|msg| {
        msg?;
        Ok(())
    })
}

/// Internal struct for helping to convert the unsafe FFI callback to the safe `Callback`.
struct CallbackContext<'a, T> {
    pub callback: Callback<T>,
    pub data: &'a mut T,
}

/// Internal FFI callback converting the callback from libmnl into a `Callback<T>` callback.
extern "C" fn callback_wrapper<T>(
    nlh: *const libc::nlmsghdr,
    data: *mut libc::c_void,
) -> libc::c_int {
    let context: &mut CallbackContext<'_, T> =
        unsafe { &mut *(data as *mut CallbackContext<'_, T>) };
    (context.callback)(unsafe { &*nlh }, context.data)
}
