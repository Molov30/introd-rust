fn main() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("{}", x);

    let x = String::from("new string");
    let y = x.clone();
    println!("x={} y={}", x, y);
}
