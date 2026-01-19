fn main() {
    let stdin = std::io::stdin();
    let mut name = String::new();
    stdin.read_line(&mut name).expect("Failed to read line");
    greet(&name);

    let mut arr = vec![5, 2, 8, 1, 9];
    arr = sort(arr);
    println!("Sorted array: {:?}", arr);
}

fn greet(name: &str) {
    print!("Hello, {}!", name);
}

fn sort<T: Ord>(arr: Vec<T>) -> Vec<T> {
    let mut sorted = arr;
    for i in 0..sorted.len() {
        for j in 0..sorted.len() - i - 1 {
            if sorted[j] > sorted[j + 1] {
                sorted.swap(j, j + 1);
            }
        }
    }
    sorted
}
