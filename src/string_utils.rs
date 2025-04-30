use std::fmt::Display;

#[repr(C)]
pub struct PaddedString<const SIZE: usize> {
    array: [u8; SIZE],
}

impl<const SIZE: usize> Display for PaddedString<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Find null terminator
        let index = self
            .array
            .iter()
            .position(|&v| v == 0)
            .unwrap_or(self.array.len());
        write!(f, "{}", String::from_utf8_lossy(&self.array[..index]))
    }
}
