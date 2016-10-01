use std::os::raw::{c_int};
use std::ffi::{CStr, CString};

use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;
use itertools::structs::Product;

pub fn sector_from_cell(cell: c_int) -> c_int {
    let mut sector = (cell + 4)/8;
    if cell < -4 { // cell + 4 < 0
        sector -= 1; // The above division rounds the wrong way (to 0), this is a hack to round down
    }
    sector
}

pub fn cells_from_sector(sector: c_int) -> Range<c_int> {
    let min = sector * 8 - 4;
    let max = min + 8;
    min .. (max + 1)
}

pub fn cell_coords_from_sector_coords(sector_x: c_int, sector_z: c_int) -> Product<Range<c_int>, Range<c_int>> {
    cells_from_sector(sector_x).cartesian_product(cells_from_sector(sector_z))
}

//pub fn zone(size: c_int) -> Product<Range<c_int>, Range<c_int>> {
    
//}

#[derive(Debug)]
pub enum Query {
    Active(ActiveQuery),
    Live(LiveUpdates),
    None
}

#[derive(Debug)]
pub struct ActiveQuery {
    /// Indicates next object is not a new cell
    pub in_cell: bool
}

impl ActiveQuery {
    pub fn new() -> Self {
        ActiveQuery {
            in_cell: false
        }
    }
}

#[derive(Debug)]
pub struct LiveUpdates {
    min_cell_x: c_int,
    max_cell_x: c_int,
    min_cell_z: c_int,
    max_cell_z: c_int
}

impl LiveUpdates {
    pub fn contains(&self, cell_x: c_int, cell_z: c_int) -> bool {
        self.min_cell_x <= cell_x && cell_x <= self.max_cell_x && self.min_cell_z <= cell_z && cell_z <= self.max_cell_z
    }
}

#[derive(Debug)]
pub struct SequenceNums(HashMap<CString, HashMap<(c_int, c_int), c_int>>);

impl SequenceNums {
    pub fn new() -> Self {
        SequenceNums(HashMap::new())
    }

    pub fn inc_and_get(&mut self, world: CString, sector_x: c_int, sector_z: c_int, start: c_int) -> c_int {
        let sequence_num = self.0.entry(world).or_insert_with(HashMap::new).entry((sector_x, sector_z)).or_insert(start);
        *sequence_num += 1;
        *sequence_num
    } 
}