use std::io;

fn main() {
    let mut result : Vec<usize> = vec![];
    let mut num:Vec<usize> = vec![];

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let times : usize = text.trim().parse().expect("Error");

    // Input One Line
    for _i in 0..times {
        let mut text2 = String::new();
        io::stdin().read_line(&mut text2).expect("no input");
        num = text2.split_whitespace()
              .map(|s|s.trim().parse().expect("Error"))
              .collect::<Vec<_>>();

        result.push(num[0] + num[1]);
    }

    for _j in 0..times{
        println!("{}", result[_j]);
    }
}