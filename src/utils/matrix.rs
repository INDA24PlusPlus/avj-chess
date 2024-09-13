// Not really a matrix but whatever
// Returns (row, col)
pub fn index_to_col_row(index: usize) -> (i32, i32) {
    if (index > 63) {
        // not continue
    }
    let row = ((index as f32) / 7.0).floor() as i32;
    let col = (index % 7) as i32;

    return (row, col);
}
