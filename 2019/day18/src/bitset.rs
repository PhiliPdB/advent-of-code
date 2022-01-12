use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet(u32);

impl BitSet {
    pub fn new() -> Self {
        Self(0)
    }

    /// Check if the bit on the specified index is set
    pub fn get(&self, index: u32) -> bool {
        (self.0 >> index) & 1 == 1
    }

    /// Set bit on specified index
    pub fn set(&mut self, index: u32) {
        self.0 |= 1 << index;
    }

    pub fn set_bits(&self) -> u32 {
        self.0.count_ones()
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
