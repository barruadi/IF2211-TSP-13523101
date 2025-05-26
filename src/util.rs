pub fn euclidean_distance(
    a: (i32, i32), 
    b: (i32, i32)
) -> usize {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (((dx * dx + dy * dy) as f64).sqrt().round()) as usize
}