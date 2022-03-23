fn main() {
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let num_year : usize = bf.trim().parse().expect("Err");

    if num_year % 4 == 0 {
        if num_year % 100 != 0 {
            print!("1");
        } else if num_year % 400 == 0 {
            print!("1");
        } else {
            print!("0");
        }
    } else { print!("0"); }
}