#[derive(Debug, Clone)]
pub struct RawDataArray {
    bits_per_entry: u8,
    inner: Vec<u64>,
    len: usize,
}

impl RawDataArray {
    pub fn new(bits_per_entry: u8) -> RawDataArray {
        RawDataArray {
            bits_per_entry,
            inner: Vec::new(),
            len: 0,
        }
    }

    pub fn inner(&self) -> &[u64] {
        &self.inner
    }

    pub fn into_inner(self) -> Vec<u64> {
        self.inner
    }

    pub fn bits_per_entry(&self) -> u8 {
        self.bits_per_entry
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn entries_per_long(&self) -> usize {
        if self.bits_per_entry == 0 {
            0
        } else {
            64 / self.bits_per_entry as usize
        }
    }

    pub fn push(&mut self, value: u64) {
        if self.bits_per_entry == 0 {
            panic!("Cannot push into an array with 0 bits per entry");
        }
        let bits = self.bits_per_entry as usize;
        let mask = (1u64 << bits) - 1;
        if value > mask {
            panic!(
                "Value {} does not fit in {} bits (max allowed is {})",
                value, bits, mask
            );
        }

        let entries_per_long = self.entries_per_long();
        let index = self.len;
        let long_index = index / entries_per_long;
        let slot = index % entries_per_long;
        let shift = slot * bits;

        // Extend the inner vector if necessary.
        if long_index >= self.inner.len() {
            self.inner.push(0);
        }
        // Insert the value by shifting it to the proper offset and OR-ing.
        self.inner[long_index] |= value << shift;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<u64> {
        if index >= self.len {
            return None;
        }
        let bits = self.bits_per_entry as usize;
        let entries_per_long = self.entries_per_long();
        let long_index = index / entries_per_long;
        let slot = index % entries_per_long;
        let shift = slot * bits;
        let mask = (1u64 << bits) - 1;
        Some((self.inner[long_index] >> shift) & mask)
    }

    pub fn set(&mut self, index: usize, value: u64) {
        if index >= self.len {
            panic!("Index {} out of bounds (length is {})", index, self.len);
        }
        let bits = self.bits_per_entry as usize;
        let mask = (1u64 << bits) - 1;
        if value > mask {
            panic!(
                "Value {} does not fit in {} bits (max allowed is {})",
                value, bits, mask
            );
        }
        let entries_per_long = self.entries_per_long();
        let long_index = index / entries_per_long;
        let slot = index % entries_per_long;
        let shift = slot * bits;

        // Clear the existing value in the target slot.
        self.inner[long_index] &= !(mask << shift);
        // Write the new value.
        self.inner[long_index] |= value << shift;
    }
}
