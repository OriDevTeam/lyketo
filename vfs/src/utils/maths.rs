// Standard Uses

// Crate Uses

// External Uses

pub fn padding_compensation(size: usize, div_by: usize) -> usize {
    (size + div_by) / div_by * div_by
}
