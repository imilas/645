pub fn vector_range(range: i32) -> Vec<i32> {
    //create a vector of numbers 0..range
    let v: Vec<i32> = (0..range).collect();
    v
}
pub fn windows(v: &Vec<i32>, overlap: i32) -> std::slice::Windows<'_, i32> {
    // windowing using vec.windows
    let iter = v.windows(overlap as usize);
    return iter;
}
fn main() {
    let v = vector_range(10);
    let iter = windows(&v, 7);
    for i in iter {
        println!("{:?}", i);
    }
}
