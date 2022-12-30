use tiled_wfc::{AxisPair, Tile as TileTrait};

#[derive(Debug)]
pub struct Tile {
    pub character: char,
    pub sockets: [AxisPair<Socket>; 2],
}

impl TileTrait<2> for Tile {
    type Socket = Socket;

    fn sockets(&self) -> [AxisPair<Self::Socket>; 2] {
        self.sockets
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Socket {
    Empty,
    Normal,
    Bold,
    Double,
}
