use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet(u32);

impl BitSet {
    pub fn new() -> Self {
        Self(0)
    }

    /// Set bit on specified index
    pub fn set(&mut self, index: u32) {
        self.0 |= 1 << index;
    }

    pub fn toggle_bit(&mut self, index: u32) {
        self.0 ^= 1 << index;
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}
