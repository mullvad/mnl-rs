use std::io;

use libc::nlmsghdr;

/// Iterator over a byte buffer of netlink messages.
#[derive(Clone)]
pub struct NlMessages<'a> {
    buffer: &'a [u8],
}

impl<'a> NlMessages<'a> {
    /// Iterate over a byte buffer of netlink messages.
    ///
    /// `buffer` must be aligned to `size_of::<nlmsghdr>()`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
}

impl<'a> Iterator for NlMessages<'a> {
    type Item = io::Result<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if size_of::<nlmsghdr>() > self.buffer.len() {
            return None;
        }

        let header = self.buffer.as_ptr().cast::<nlmsghdr>();

        if !header.is_aligned() {
            self.buffer = &[];
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Buffer is not aligned to size_of::<nlmsghdr>()",
            )));
        }

        // Safety:
        // nlmsghdr is a C struct, valid for all bit-patterns, and we've checked alignment and length
        let header = unsafe { header.read() };

        let msg_len = header.nlmsg_len as usize;

        // Validate message length
        if msg_len < size_of::<nlmsghdr>() {
            self.buffer = &[];
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Invalid netlink message length: {msg_len} (minimum is {})",
                    size_of::<nlmsghdr>(),
                ),
            )));
        }

        if msg_len > self.buffer.len() {
            self.buffer = &[];
            return Some(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Message length {msg_len} exceeds remaining buffer size {}",
                    self.buffer.len(),
                ),
            )));
        }

        // Extract this message
        let padded_msg_len = msg_len.next_multiple_of(align_of::<nlmsghdr>());
        let (message_with_padding, remaining) = self.buffer.split_at(padded_msg_len);
        let (message, _padding) = message_with_padding.split_at(msg_len);

        // Move to next message
        self.buffer = remaining;

        Some(Ok(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_message() {
        // Single 36-byte NLMSG_ERROR message (typical ACK)
        let mut buffer = vec![0u8; 36];
        buffer[0..4].copy_from_slice(&36u32.to_ne_bytes()); // nlmsg_len

        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].len(), 36);
    }

    #[test]
    fn test_parse_multiple_messages() {
        // Two 36-byte messages back-to-back (4-byte aligned)
        let mut buffer = vec![0u8; 72];
        buffer[0..4].copy_from_slice(&36u32.to_ne_bytes());
        buffer[36..40].copy_from_slice(&36u32.to_ne_bytes());

        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].len(), 36);
        assert_eq!(messages[1].len(), 36);
    }

    #[test]
    fn test_parse_aligned_messages() {
        // Message with length 35, should be aligned to 36
        let mut buffer = vec![0u8; 36];
        buffer[0..4].copy_from_slice(&35u32.to_ne_bytes());

        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].len(), 35); // Message slice is exact length
    }

    #[test]
    fn test_parse_three_messages_with_alignment() {
        // Three messages: 36, 35 (aligned to 36), 36 bytes
        let mut buffer = vec![0u8; 108];
        buffer[0..4].copy_from_slice(&36u32.to_ne_bytes());
        buffer[36..40].copy_from_slice(&35u32.to_ne_bytes());
        buffer[72..76].copy_from_slice(&36u32.to_ne_bytes());

        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].len(), 36);
        assert_eq!(messages[1].len(), 35);
        assert_eq!(messages[2].len(), 36);
    }

    #[test]
    fn test_parse_empty_buffer() {
        let buffer = vec![];
        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_parse_incomplete_header() {
        // Buffer too small for a complete nlmsghdr (need 16 bytes minimum)
        let buffer = vec![0u8; 10];
        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 0); // Should skip incomplete message
    }

    #[test]
    fn test_parse_invalid_length_too_small() {
        // Message claims to be 10 bytes (less than minimum 16)
        let mut buffer = vec![0u8; 20];
        buffer[0..4].copy_from_slice(&10u32.to_ne_bytes());

        let result: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid netlink message length")
        );
    }

    #[test]
    fn test_parse_invalid_length_exceeds_buffer() {
        // Message claims to be 100 bytes but buffer is only 50
        let mut buffer = vec![0u8; 50];
        buffer[0..4].copy_from_slice(&100u32.to_ne_bytes());

        let result: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("exceeds remaining buffer size")
        );
    }

    #[test]
    fn test_parse_zero_length() {
        // Message with length 0
        let mut buffer = vec![0u8; 20];
        buffer[0..4].copy_from_slice(&0u32.to_ne_bytes());

        let result: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_minimum_valid_message() {
        // Minimum valid netlink message is 16 bytes (just the header)
        let mut buffer = vec![0u8; 16];
        buffer[0..4].copy_from_slice(&16u32.to_ne_bytes());

        let messages: Result<Vec<_>, _> = NlMessages::new(&buffer).collect();
        let messages = messages.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].len(), 16);
    }
}
