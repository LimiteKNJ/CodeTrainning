use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Error");
    let c :char = text.trim().parse().expect("Error");
    let ascii = c as u32;

    print!("{}", ascii);
}
