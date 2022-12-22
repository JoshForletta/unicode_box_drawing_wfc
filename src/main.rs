use unicode_box_drawing_wfc::TILES;
use wfc::Matrix;

fn main() {
    let mut matrix = Matrix::builder()
        .tile_set(TILES)
        .dimensions([120, 40])
        .build()
        .unwrap();

    matrix.collapse();

    for states in matrix.get_matrix().chunks(matrix.get_dimensions()[0]).rev() {
        let line: String = states
            .into_iter()
            .map(|state| matrix.get_tile(*state).unwrap().character)
            .collect();

        println!("{}", line);
    }
}
