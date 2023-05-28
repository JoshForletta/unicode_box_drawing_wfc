use crate::{tags, UBDSocket, UBDTile};
use tags::*;
use UBDSocket::*;

macro_rules! sockets {
    ($pos_x:expr, $neg_x:expr, $pos_y:expr, $neg_y:expr) => {
        [
            tiled_wfc::AxisPair::new($pos_x, $neg_x),
            tiled_wfc::AxisPair::new($pos_y, $neg_y),
        ]
    };
}

macro_rules! tags {
    ($tag:expr$(, $n_tag:expr)*) => {
        $tag $(| $n_tag)*
    };
}

pub const TILE_SET: &[UBDTile] = &[
    UBDTile::new(
        ' ',
        sockets!(Empty, Empty, Empty, Empty),
        tags![EMPTY, BLANK],
    ),
    UBDTile::new(
        '─',
        sockets!(Normal, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, STRAIGHT],
    ),
    UBDTile::new(
        '━',
        sockets!(Bold, Bold, Empty, Empty),
        tags![EMPTY, BOLD, STRAIGHT],
    ),
    UBDTile::new(
        '│',
        sockets!(Empty, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, STRAIGHT],
    ),
    UBDTile::new(
        '┃',
        sockets!(Empty, Empty, Bold, Bold),
        tags![EMPTY, BOLD, STRAIGHT],
    ),
    UBDTile::new(
        '╌',
        sockets!(Normal, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_TWO],
    ),
    UBDTile::new(
        '╍',
        sockets!(Bold, Bold, Empty, Empty),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_TWO],
    ),
    UBDTile::new(
        '╎',
        sockets!(Empty, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_TWO],
    ),
    UBDTile::new(
        '╏',
        sockets!(Empty, Empty, Bold, Bold),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_TWO],
    ),
    UBDTile::new(
        '┄',
        sockets!(Normal, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_THREE],
    ),
    UBDTile::new(
        '┅',
        sockets!(Bold, Bold, Empty, Empty),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_THREE],
    ),
    UBDTile::new(
        '┆',
        sockets!(Empty, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_THREE],
    ),
    UBDTile::new(
        '┇',
        sockets!(Empty, Empty, Bold, Bold),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_THREE],
    ),
    UBDTile::new(
        '┈',
        sockets!(Normal, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_FOUR],
    ),
    UBDTile::new(
        '┉',
        sockets!(Bold, Bold, Empty, Empty),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_FOUR],
    ),
    UBDTile::new(
        '┊',
        sockets!(Empty, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, STRAIGHT, DASHED_FOUR],
    ),
    UBDTile::new(
        '┋',
        sockets!(Empty, Empty, Bold, Bold),
        tags![EMPTY, BOLD, STRAIGHT, DASHED_FOUR],
    ),
    UBDTile::new(
        '┌',
        sockets!(Normal, Empty, Empty, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┍',
        sockets!(Bold, Empty, Empty, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┎',
        sockets!(Normal, Empty, Empty, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┏',
        sockets!(Bold, Empty, Empty, Bold),
        tags![EMPTY, BOLD, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┐',
        sockets!(Empty, Normal, Empty, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┑',
        sockets!(Empty, Bold, Empty, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┒',
        sockets!(Empty, Normal, Empty, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┓',
        sockets!(Empty, Bold, Empty, Bold),
        tags![EMPTY, BOLD, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '└',
        sockets!(Normal, Empty, Normal, Empty),
        tags![EMPTY, NORMAL, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┕',
        sockets!(Bold, Empty, Normal, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┖',
        sockets!(Normal, Empty, Bold, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┗',
        sockets!(Bold, Empty, Bold, Empty),
        tags![EMPTY, BOLD, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┘',
        sockets!(Empty, Normal, Normal, Empty),
        tags![EMPTY, NORMAL, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '┙',
        sockets!(Empty, Bold, Normal, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┚',
        sockets!(Empty, Normal, Bold, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '┛',
        sockets!(Empty, Bold, Bold, Empty),
        tags![EMPTY, BOLD, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '├',
        sockets!(Normal, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┝',
        sockets!(Bold, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┞',
        sockets!(Normal, Empty, Bold, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┟',
        sockets!(Normal, Empty, Normal, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┠',
        sockets!(Normal, Empty, Bold, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┡',
        sockets!(Bold, Empty, Bold, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┢',
        sockets!(Bold, Empty, Normal, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┣',
        sockets!(Bold, Empty, Bold, Bold),
        tags![EMPTY, BOLD, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┤',
        sockets!(Empty, Normal, Normal, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┥',
        sockets!(Empty, Bold, Normal, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┦',
        sockets!(Empty, Normal, Bold, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┧',
        sockets!(Empty, Normal, Normal, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┨',
        sockets!(Empty, Normal, Bold, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┩',
        sockets!(Empty, Bold, Bold, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┪',
        sockets!(Empty, Bold, Normal, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┫',
        sockets!(Empty, Bold, Bold, Bold),
        tags![EMPTY, BOLD, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┬',
        sockets!(Normal, Normal, Empty, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┭',
        sockets!(Normal, Bold, Empty, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┮',
        sockets!(Bold, Normal, Empty, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┯',
        sockets!(Bold, Bold, Empty, Normal),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┰',
        sockets!(Normal, Normal, Empty, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┱',
        sockets!(Normal, Bold, Empty, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┲',
        sockets!(Bold, Normal, Empty, Bold),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┳',
        sockets!(Bold, Bold, Empty, Bold),
        tags![EMPTY, BOLD, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┴',
        sockets!(Normal, Normal, Normal, Empty),
        tags![EMPTY, NORMAL, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┵',
        sockets!(Normal, Bold, Normal, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┶',
        sockets!(Bold, Normal, Normal, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┷',
        sockets!(Bold, Bold, Normal, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┸',
        sockets!(Normal, Normal, Bold, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┹',
        sockets!(Normal, Bold, Bold, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┺',
        sockets!(Bold, Normal, Bold, Empty),
        tags![EMPTY, NORMAL, BOLD, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '┻',
        sockets!(Bold, Bold, Bold, Empty),
        tags![EMPTY, BOLD, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '┼',
        sockets!(Normal, Normal, Normal, Normal),
        tags![NORMAL, INTERSECTION_FOUR],
    ),
    UBDTile::new(
        '┽',
        sockets!(Normal, Bold, Normal, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '┾',
        sockets!(Bold, Normal, Normal, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '┿',
        sockets!(Bold, Bold, Normal, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╀',
        sockets!(Normal, Normal, Bold, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╁',
        sockets!(Normal, Normal, Normal, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╂',
        sockets!(Normal, Normal, Bold, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╃',
        sockets!(Normal, Bold, Bold, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╄',
        sockets!(Bold, Normal, Bold, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╅',
        sockets!(Normal, Bold, Normal, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╆',
        sockets!(Bold, Normal, Normal, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╇',
        sockets!(Bold, Bold, Bold, Normal),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╈',
        sockets!(Bold, Bold, Normal, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╉',
        sockets!(Normal, Bold, Bold, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╊',
        sockets!(Bold, Normal, Bold, Bold),
        tags![NORMAL, BOLD, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╋',
        sockets!(Bold, Bold, Bold, Bold),
        tags![BOLD, INTERSECTION_FOUR],
    ),
    UBDTile::new(
        '═',
        sockets!(Double, Double, Empty, Empty),
        tags![EMPTY, DOUBLE, STRAIGHT],
    ),
    UBDTile::new(
        '║',
        sockets!(Empty, Empty, Double, Double),
        tags![EMPTY, DOUBLE, STRAIGHT],
    ),
    UBDTile::new(
        '╒',
        sockets!(Double, Empty, Empty, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╓',
        sockets!(Normal, Empty, Empty, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╔',
        sockets!(Double, Empty, Empty, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '╕',
        sockets!(Empty, Double, Empty, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╖',
        sockets!(Empty, Normal, Empty, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╗',
        sockets!(Empty, Double, Empty, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '╘',
        sockets!(Double, Empty, Normal, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╙',
        sockets!(Normal, Empty, Double, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╚',
        sockets!(Double, Empty, Double, Empty),
        tags![EMPTY, DOUBLE, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '╛',
        sockets!(Empty, Double, Normal, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╜',
        sockets!(Empty, Normal, Double, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_TWO, MIXED],
    ),
    UBDTile::new(
        '╝',
        sockets!(Empty, Double, Double, Empty),
        tags![EMPTY, DOUBLE, INTERSECTION_TWO],
    ),
    UBDTile::new(
        '╞',
        sockets!(Double, Empty, Normal, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╟',
        sockets!(Normal, Empty, Double, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╠',
        sockets!(Double, Empty, Double, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '╡',
        sockets!(Empty, Double, Normal, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╢',
        sockets!(Empty, Normal, Double, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╣',
        sockets!(Empty, Double, Double, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '╤',
        sockets!(Double, Double, Empty, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╥',
        sockets!(Normal, Normal, Empty, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╦',
        sockets!(Double, Double, Empty, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '╧',
        sockets!(Double, Double, Normal, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╨',
        sockets!(Normal, Normal, Double, Empty),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_THREE, MIXED],
    ),
    UBDTile::new(
        '╩',
        sockets!(Double, Double, Double, Empty),
        tags![EMPTY, DOUBLE, INTERSECTION_THREE],
    ),
    UBDTile::new(
        '╪',
        sockets!(Double, Double, Normal, Normal),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╫',
        sockets!(Normal, Normal, Double, Double),
        tags![EMPTY, NORMAL, DOUBLE, INTERSECTION_FOUR, MIXED],
    ),
    UBDTile::new(
        '╬',
        sockets!(Double, Double, Double, Double),
        tags![EMPTY, DOUBLE, INTERSECTION_FOUR],
    ),
    UBDTile::new(
        '╭',
        sockets!(Normal, Empty, Empty, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_TWO, ROUNDED],
    ),
    UBDTile::new(
        '╮',
        sockets!(Empty, Normal, Empty, Normal),
        tags![EMPTY, NORMAL, INTERSECTION_TWO, ROUNDED],
    ),
    UBDTile::new(
        '╯',
        sockets!(Empty, Normal, Normal, Empty),
        tags![EMPTY, NORMAL, INTERSECTION_TWO, ROUNDED],
    ),
    UBDTile::new(
        '╰',
        sockets!(Normal, Empty, Normal, Empty),
        tags![EMPTY, NORMAL, INTERSECTION_TWO, ROUNDED],
    ),
    UBDTile::new(
        '╴',
        sockets!(Empty, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, END_STOP],
    ),
    UBDTile::new(
        '╵',
        sockets!(Empty, Empty, Normal, Empty),
        tags![EMPTY, NORMAL, END_STOP],
    ),
    UBDTile::new(
        '╶',
        sockets!(Normal, Empty, Empty, Empty),
        tags![EMPTY, NORMAL, END_STOP],
    ),
    UBDTile::new(
        '╷',
        sockets!(Empty, Empty, Empty, Normal),
        tags![EMPTY, NORMAL, END_STOP],
    ),
    UBDTile::new(
        '╸',
        sockets!(Empty, Bold, Empty, Empty),
        tags![EMPTY, BOLD, END_STOP],
    ),
    UBDTile::new(
        '╹',
        sockets!(Empty, Empty, Bold, Empty),
        tags![EMPTY, BOLD, END_STOP],
    ),
    UBDTile::new(
        '╺',
        sockets!(Bold, Empty, Empty, Empty),
        tags![EMPTY, BOLD, END_STOP],
    ),
    UBDTile::new(
        '╻',
        sockets!(Empty, Empty, Empty, Bold),
        tags![EMPTY, BOLD, END_STOP],
    ),
    UBDTile::new(
        '╼',
        sockets!(Bold, Normal, Empty, Empty),
        tags![EMPTY, NORMAL, BOLD, TRANSITION, MIXED],
    ),
    UBDTile::new(
        '╽',
        sockets!(Empty, Empty, Normal, Bold),
        tags![EMPTY, NORMAL, BOLD, TRANSITION, MIXED],
    ),
    UBDTile::new(
        '╾',
        sockets!(Normal, Bold, Empty, Empty),
        tags![EMPTY, NORMAL, BOLD, TRANSITION, MIXED],
    ),
    UBDTile::new(
        '╿',
        sockets!(Empty, Empty, Bold, Normal),
        tags![EMPTY, NORMAL, BOLD, TRANSITION, MIXED],
    ),
];
