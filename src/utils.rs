pub fn repeat<T: Clone>(x: T, y: u8) -> Vec<T> {
    vec![x; y as usize]
}
