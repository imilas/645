fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn xget(&self) -> &T {
        &self.x
    }
}
pub fn vector_range(range: i32) {
    let v: Vec<i32> = (0..range).collect();
    println!("{:?}", v);
}

fn main() {
    let number_list = vec![1, 4, 5, 7, 4, 2, 6];
    let result = largest(&number_list);
    println!("{}", result);

    let i = Point { x: 4.0, y: 2.0 };
    println!("{i:#?}");
    println!("{}", i.x);

    vector_range(10);
}
