// mod linked_list;

const headers: Box<Vec<&str>> = Box::new(vec!["A", "B", "C"]);

fn main() {
    println!("Hello, world!");
    headers.iter()
        .for_each(|&header| println!("header = [{}]", header));
}
