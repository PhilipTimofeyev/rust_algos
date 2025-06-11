mod binary_search;

fn main() {
    let result = binary_search::binary_search(vec![1, 2, 3, 4], -10);
    println!("{}", result)
}
