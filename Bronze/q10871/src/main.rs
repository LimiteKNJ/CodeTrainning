use std::io;

fn main() {
    // Input One Line
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let num:Vec<usize> = text.split_whitespace()
                                            .map(|s|s.trim().parse().expect("parsing error"))
                                            .collect::<Vec<_>>();
    let size = num[0];

    let mut text2 = String::new();
    io::stdin().read_line(&mut text2).expect("no input");
    let num2:Vec<usize> = text2.split_whitespace()
                                            .map(|s|s.trim().parse().expect("parsing error"))
                                            .collect::<Vec<_>>();                                 

    for i in 0..size {
        if num2[i] < num[1] {
            print!("{} ", num2[i]);
        }
    }
}