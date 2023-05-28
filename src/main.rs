use std::{thread::sleep, time::Duration};

use rand::{rngs::StdRng, SeedableRng};
use tiled_wfc::{Collapser, Picker, RandLeastEntropicPicker, State, WFC};
use unicode_box_drawing_wfc::{
    tags::{
        DASHED_FOUR, DASHED_THREE, DASHED_TWO, DOUBLE, END_STOP, INTERSECTION_FOUR,
        INTERSECTION_THREE, INTERSECTION_TWO, MIXED, NORMAL, ROUNDED, STRAIGHT, TRANSITION,
    },
    BDSocket, BDTile, TILE_SET,
};

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

fn background_color(r: u8, g: u8, b: u8) {
    print!("\u{1b}[48;2;{};{};{}m", r, g, b);
}

fn reset() {
    print!("\u{1b}[m");
}

fn print_state(tile_set: &[BDTile], state: &State) {
    match state {
        State::Collapsed(state) => print!("{}", tile_set[*state].character),
        State::Superimposed(state) => match state.count() {
            0 => {
                background_color(255, 0, 0);
                print!(" ");
                reset();
            }
            count => {
                let count_ratio = (count as f32 / tile_set.len() as f32).clamp(0.0, 1.0);
                let steped = (count_ratio * 3.0).round();
                let index = (steped - 1.0) as usize;
                let shades = ['▓', '▒', '░'];

                print!("{}", shades[index]);
            }
        },
    }
}

fn print_matrix<'a, C, P>(wfc: &WFC<'a, BDTile, 2, C, P>)
where
    C: Collapser,
    P: Picker<2>,
{
    let lines = wfc
        .matrix()
        .matrix()
        .chunks(wfc.matrix().dimensions()[0])
        .rev();

    for line in lines {
        for state in line {
            print_state(wfc.tile_set(), state);
        }
        println!();
    }
}

fn main() {
    let (width, height) = term_size::dimensions().unwrap_or((139, 44));

    let tile_set: Vec<_> = TILE_SET
        .into_iter()
        .filter(|tile| !tile.is(DOUBLE))
        .filter(|tile| !tile.is(TRANSITION))
        .filter(|tile| !tile.is(END_STOP))
        .filter(|tile| !tile.is(MIXED))
        // .filter(|tile| !tile.is(NORMAL | STRAIGHT))
        .filter(|tile| !(tile.is(NORMAL | INTERSECTION_TWO) && !tile.is(ROUNDED)))
        // .filter(|tile| !tile.is(INTERSECTION_TWO))
        .filter(|tile| !tile.is(INTERSECTION_FOUR))
        .filter(|tile| !tile.is(INTERSECTION_THREE))
        // .filter(|tile| !tile.is(ROUNDED))
        // .filter(|tile| !(tile.is(DASHED_TWO) || tile.is(DASHED_THREE) || tile.is(DASHED_FOUR)))
        .copied()
        .collect();

    let mut wfc = WFC::builder()
        .tile_set(tile_set.as_slice())
        .dimensions([width, height])
        .picker(RandLeastEntropicPicker::new(StdRng::from_entropy()))
        // .dimensions([80, 40])
        // .seed_from_u64(422)
        .build();

    enable_alternate_screen_buffer();
    clear();
    hide_cursor();

    let mut collapse_records = Vec::new();

    while let Some(index) = wfc.pick() {
        sleep(Duration::from_millis(2));

        match wfc.collapse_state(index) {
            Ok(collapse_record) => collapse_records.push(collapse_record),
            Err(_) => loop {
                let collapse_record = collapse_records.pop().expect("solution");

                if wfc.uncollapse_state(collapse_record).is_ok() {
                    break;
                }

                print_matrix(&wfc);

                sleep(Duration::from_millis(2));
            },
        }

        print_matrix(&wfc);
    }

    clear();
    disable_alternate_screen_buffer();

    print_matrix(&wfc);
    show_cursor();
}
