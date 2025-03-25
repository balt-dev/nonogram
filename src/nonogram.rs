//! Holds the Nonogram struct.

/// Structure representing a nonogram's grid.
pub struct NonogramGrid {
    known_cells: [u128; 128],
    cell_values: [u128; 128],
    width: usize,
    height: usize
}

#[derive(Copy, Clone)]
pub struct IndexError { x: usize, y: usize, width: usize, height: usize }

impl std::fmt::Debug for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }    
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index [{}, {}] out of bounds for grid of size [{}, {}]", self.x, self.y, self.width, self.height)
    }
}

const fn get_bit(buf: u128, mut index: usize) -> bool {
    index %= 128;
    (buf >> index) & 1 == 1
}

const fn set_bit(buf: &mut u128, mut index: usize, value: bool) {
    index %= 128;
    *buf &= !(1 << index);
    *buf |= (value as u128) << index;
}


impl NonogramGrid {
    /// Creates a new nonogram grid filled with unknown values.
    ///
    /// Will return [`None`] if width or height are greater than 128.
    pub const fn new(width: usize, height: usize) -> Option<Self> {
        if width > 128 || height > 128 { return None; }
        Some(Self { width, height, known_cells: [0; 128], cell_values: [0; 128] })
    }

    /// Returns the value at a given cell.
    /// 
    /// # Errors
    /// Returns [`IndexError`] if the cell is out of bounds.
    pub const fn get(&self, x: usize, y: usize) -> Result<Option<bool>, IndexError> {
        if x >= self.width || y >= self.height { return Err(IndexError { x, y, width: self.width, height: self.height }); }
        return if get_bit(self.known_cells[y], x) { 
            Ok(Some(get_bit(self.cell_values[y], x)))
        } else { Ok(None) }
    }

    /// Sets the value at a given cell.
    /// 
    /// # Errors
    /// Returns [`IndexError`] if the cell is out of bounds.
    pub const fn set(&mut self, x: usize, y: usize, value: Option<bool>) -> Result<(), IndexError> {
        if x >= self.width || y >= self.height { return Err(IndexError { x, y, width: self.width, height: self.height }); }
        if let Some(v) = value {
            set_bit(&mut self.known_cells[y], x, true);
            set_bit(&mut self.cell_values[y], x, v);
        } else {
            set_bit(&mut self.known_cells[y], x, false);
        }
        Ok(())
    }

    
}