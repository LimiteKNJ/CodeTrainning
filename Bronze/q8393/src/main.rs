use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let num : usize = text.trim().parse().expect("Error");
    let mut res = 0;

    for i in 1..num+1{
        res = res + i;
    } println!("{}", res);
}
