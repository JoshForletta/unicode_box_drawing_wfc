use tiled_wfc::{AxisPair, Tile as TileTrait};

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub character: char,
    pub sockets: [AxisPair<Socket>; 2],
}

impl Tile {
    pub const fn new(
        character: char,
        pos_x: Socket,
        neg_x: Socket,
        pos_y: Socket,
        neg_y: Socket,
    ) -> Self {
        Self {
            character,
            sockets: [AxisPair::new(pos_x, neg_x), AxisPair::new(pos_y, neg_y)],
        }
    }
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
    Block((bool, bool)),
}
