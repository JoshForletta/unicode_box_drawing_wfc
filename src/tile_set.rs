use tiled_wfc::AxisPair;

use crate::{Socket, Tile};
use Socket::*;

pub const TILES: &[Tile] = &[
    Tile {
        character: ' ',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '─',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '━',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '│',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┃',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┄',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┅',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┆',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┇',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┈',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┉',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┊',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┋',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┌',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┍',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┎',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┏',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┐',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┑',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┒',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┓',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '└',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┕',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┖',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┗',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┘',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┙',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┚',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┛',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '├',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┝',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┞',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┟',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┠',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┡',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┢',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┣',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┤',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┥',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┦',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┧',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┨',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┩',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┪',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┫',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┬',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┭',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┮',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┯',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┰',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┱',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┲',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┳',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '┴',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┵',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┶',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┷',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┸',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┹',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┺',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┻',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '┼',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┽',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┾',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '┿',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╀',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╁',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╂',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╃',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╄',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╅',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╆',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╇',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╈',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╉',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╊',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Normal,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╋',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '╌',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╍',
        sockets: [
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╎',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╏',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Bold,
                neg: Bold,
            },
        ],
    },
    Tile {
        character: '═',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '║',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Empty,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╒',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╓',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╔',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╕',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╖',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╗',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╘',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╙',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╚',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╛',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╜',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╝',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╞',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╟',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╠',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Empty,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╡',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╢',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╣',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Double,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╤',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╥',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╦',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Empty,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╧',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╨',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╩',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Double,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╪',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╫',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Normal,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╬',
        sockets: [
            AxisPair {
                pos: Double,
                neg: Double,
            },
            AxisPair {
                pos: Double,
                neg: Double,
            },
        ],
    },
    Tile {
        character: '╭',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╮',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
        ],
    },
    Tile {
        character: '╯',
        sockets: [
            AxisPair {
                pos: Empty,
                neg: Normal,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    Tile {
        character: '╰',
        sockets: [
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
            AxisPair {
                pos: Normal,
                neg: Empty,
            },
        ],
    },
    // Tile {
    //     character: '╴',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Normal,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╵',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Normal,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╶',
    //     sockets: [
    //         AxisPair {
    //             pos: Normal,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╷',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Normal,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╸',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Bold,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╹',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Bold,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╺',
    //     sockets: [
    //         AxisPair {
    //             pos: Bold,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╻',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Bold,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╼',
    //     sockets: [
    //         AxisPair {
    //             pos: Bold,
    //             neg: Normal,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╽',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Normal,
    //             neg: Bold,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╾',
    //     sockets: [
    //         AxisPair {
    //             pos: Normal,
    //             neg: Bold,
    //         },
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //     ],
    // },
    // Tile {
    //     character: '╿',
    //     sockets: [
    //         AxisPair {
    //             pos: Empty,
    //             neg: Empty,
    //         },
    //         AxisPair {
    //             pos: Bold,
    //             neg: Normal,
    //         },
    //     ],
    // },
];
