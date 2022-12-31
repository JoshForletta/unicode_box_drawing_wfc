use std::{thread::sleep, time::Duration};

use rand::{rngs::StdRng, seq::{SliceRandom, IteratorRandom}, Rng, SeedableRng};
use tiled_wfc::{Collapser, WFC};
use unicode_box_drawing_wfc::{Socket, Tile, TILES};

fn clear() {
    print!("\u{1b}[H");
    print!("\u{1b}[J");
}

fn enable_alternate_screen_buffer() {
    print!("\u{1b}[s");
    print!("\u{1b}[?1049h");
    clear();
}

fn disable_alternate_screen_buffer() {
    print!("\u{1b}[?1049l");
    print!("\u{1b}[u");
}

fn hide_cursor() {
    print!("\u{1b}[?25l");
}

fn show_cursor() {
    print!("\u{1b}[?25h");
}

fn blacklist_blocks_filter(tile: &Tile) -> bool {
    let match_blocks = |socket: Socket| match socket {
        Socket::Block(_) => true,
        _ => false,
    };

    for axis_pair in tile.sockets.into_iter() {
        if match_blocks(axis_pair.pos) || match_blocks(axis_pair.neg) {
            return false;
        }
    }

    true
}

fn blacklist_socket(tile: &Tile, socket: Socket) -> bool {
    !(tile.sockets[0].pos == socket
        || tile.sockets[0].neg == socket
        || tile.sockets[1].pos == socket
        || tile.sockets[1].neg == socket)
}

fn least_entropic_index<'a, C>(wfc: &WFC<'a, Tile, 2, C>, rng: &mut StdRng) -> Option<usize>
where
    C: Collapser,
{
    let mut min_count = None;
    let mut min_indexes = None;

    for (index, count) in wfc.matrix().into_iter().map(|state| state.count()).enumerate() {
        if count < 2 {
            continue;
        }

        if min_count.is_none() || count < min_count.unwrap() {
            min_count = Some(count);
            min_indexes = Some(Vec::from([index]));
            continue;
        }

        if count == min_count.unwrap() {
            min_indexes.as_mut().unwrap().push(index);
        }
    }

    min_indexes?.into_iter().choose(rng)
}

fn random_index<'a, C>(wfc: &WFC<'a, Tile, 2, C>, rng: &mut StdRng) -> Option<usize>
where
    C: Collapser,
{
    wfc.matrix().into_iter().enumerate().filter(|(_, state)| state.count() != 1).choose(rng).map(|(index, _)| index)
}

fn print_matrix<'a, C>(wfc: &WFC<'a, Tile, 2, C>)
where
    C: Collapser,
{
    let lines = wfc.matrix().matrix().chunks(wfc.dimensions()[0]).rev();
    let last_index = lines.len() - 1;

    for (index, states) in lines.enumerate() {
        let line: String = states
            .into_iter()
            .map(|state| wfc.get_tile(state).map_or(' ', |tile| tile.character))
            .collect();

        print!("{}", line);

        if index < last_index {
            println!();
        }
    }
}

fn main() {
    let (width, height) = term_size::dimensions().unwrap();

    let tile_set: Vec<_> = TILES
        .into_iter()
        .filter(|tile| blacklist_blocks_filter(tile))
        // .filter(|tile| blacklist_socket(tile, Socket::Normal))
        // .filter(|tile| !blacklist_socket(tile, Socket::Double))
        .copied()
        .collect();

    let mut wfc = WFC::builder()
        .tile_set(tile_set.as_slice())
        .dimensions([width, height])
        // .seed(69)
        .build()
        .unwrap();

    let mut rng = StdRng::from_entropy();
    let collapser = wfc.collapser().clone();
    let matrix_size = wfc.matrix().matrix().len();

    enable_alternate_screen_buffer();
    hide_cursor();

    for _ in 0..((width * height) / 100) {
        let index = rng.gen_range(0..matrix_size);
        wfc.get_mut(index)
            .unwrap()
            .collapse(&collapser, &mut rng)
            .unwrap();
        wfc.propagate(index).unwrap();

        clear();
        print_matrix(&wfc);

        sleep(Duration::from_millis(20));
    }

    while let Some(index) = least_entropic_index(&wfc, &mut rng) {
        wfc.get_mut(index)
            .expect("`self.least_entropic_index()` should return a valid index")
            .collapse(&collapser, &mut rng)
            .unwrap();

        wfc.propagate(index).unwrap();

        clear();
        print_matrix(&wfc);

        sleep(Duration::from_millis(1));
    }

    disable_alternate_screen_buffer();
    show_cursor();

    print_matrix(&wfc);
}
