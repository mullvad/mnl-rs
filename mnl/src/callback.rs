use mnl_sys;
use mnl_sys::libc::c_void;

use std::io;
use std::ptr;


/// The result of processing a batch of netlink responses.
pub enum CbResult {
    /// Everything went fine and this batch is finished processing.
    Stop,
    /// Everything went fine, but we expect more messages to come back from the kernel for this
    /// batch.
    Ok,
}

/// Callback runqueue for netlink messages. Checks that all netlink messages in `buffer` are OK.
pub fn cb_run(buffer: &[u8], seq: u32, portid: u32) -> io::Result<CbResult> {
    let len = buffer.len();
    let buf = buffer.as_ptr() as *const c_void;
    debug!("Processing {} byte netlink message", len);
    match unsafe { mnl_sys::mnl_cb_run(buf, len, seq, portid, None, ptr::null_mut()) } {
        i if i <= -1 => Err(io::Error::last_os_error()),
        mnl_sys::MNL_CB_STOP => Ok(CbResult::Stop),
        _ => Ok(CbResult::Ok),
    }
}
