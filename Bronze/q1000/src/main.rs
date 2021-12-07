use std::io;

fn main() {
    // Input One Line
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let num:Vec<usize> = text.split_whitespace()
                                            .map(|s|s.trim().parse().expect("parsing error"))
                                            .collect::<Vec<_>>();
    
    let result = num[0] + num[1];
    println!("{}", result);
}
