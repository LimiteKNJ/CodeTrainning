use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let mut num: Vec<usize> = text.split_whitespace()
                            .map(|s|s.trim().parse().expect("Error"))
                            .collect::<Vec<_>>();

    let result = num[0] * num[1];
    println!("{}", result);
}
