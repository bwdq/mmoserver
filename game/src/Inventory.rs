
pub struct inventory {
    inv_type: u8,
    size_x: u8,
    size_y: u8,
    contents: vec![vec![0; width]; height]
}