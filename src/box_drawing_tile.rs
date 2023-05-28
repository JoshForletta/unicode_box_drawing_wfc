use tiled_wfc::{AxisPair, Tile};

#[derive(Debug, Clone, Copy)]
pub struct UBDTile {
    pub character: char,
    pub sockets: [AxisPair<UBDSocket>; 2],
    pub tags_mask: usize,
}

impl UBDTile {
    pub const fn new(character: char, sockets: [AxisPair<UBDSocket>; 2], tags_mask: usize) -> Self {
        Self {
            character,
            sockets,
            tags_mask,
        }
    }

    /// Returns [`true`] if `self.tags_mask` fits `tags_mask`.
    pub const fn is(&self, tags_mask: usize) -> bool {
        self.tags_mask & tags_mask == tags_mask
    }
}

impl Tile<2> for UBDTile {
    type Socket = UBDSocket;

    fn sockets(&self) -> [AxisPair<Self::Socket>; 2] {
        self.sockets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UBDSocket {
    Empty,
    Normal,
    Bold,
    Double,
}

pub mod tags {
    pub const EMPTY: usize = 1 << 0;
    pub const NORMAL: usize = 1 << 1;
    pub const BOLD: usize = 1 << 2;
    pub const DOUBLE: usize = 1 << 3;
    pub const STRAIGHT: usize = 1 << 4;
    pub const INTERSECTION_TWO: usize = 1 << 5;
    pub const INTERSECTION_THREE: usize = 1 << 6;
    pub const INTERSECTION_FOUR: usize = 1 << 7;
    pub const DASHED_TWO: usize = 1 << 8;
    pub const DASHED_THREE: usize = 1 << 9;
    pub const DASHED_FOUR: usize = 1 << 10;
    pub const BLANK: usize = 1 << 11;
    pub const ROUNDED: usize = 1 << 12;
    pub const MIXED: usize = 1 << 13;
    pub const END_STOP: usize = 1 << 14;
    pub const TRANSITION: usize = 1 << 15;
}
