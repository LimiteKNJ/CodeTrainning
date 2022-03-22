use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Error");
    let num:Vec<isize> = text.split_whitespace()
                            .map(|s|s.trim().parse().expect("parsing error"))
                            .collect::<Vec<_>>();

    if num[0] == num[1] {
        print!("==");
    } else if num[0] > num[1] {
        print!(">");
    } else if num[0] < num[1] {
        print!("<");
    } else { print!("Err");}
}
