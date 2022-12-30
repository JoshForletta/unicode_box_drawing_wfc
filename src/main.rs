use tiled_wfc::WFC;
use unicode_box_drawing_wfc::TILES;

fn main() {
    let mut matrix = WFC::builder()
        .tile_set(TILES)
        .dimensions([120, 40])
        // .seed(69)
        .build()
        .unwrap();

    matrix.collapse().unwrap();

    for states in matrix.matrix().chunks(matrix.dimensions()[0]).rev() {
        let line: String = states
            .into_iter()
            .map(|state| matrix.get_tile(state).unwrap().character)
            .collect();

        println!("{}", line);
    }
}
