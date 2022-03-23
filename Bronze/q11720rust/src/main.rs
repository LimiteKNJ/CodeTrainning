fn main() {
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let size : usize = bf.trim().parse().expect("Err");

    let mut bf2 = String::new();
    stdin.read_line(&mut bf2).expect("Err");
    let num : Vec<char> = bf2.chars().collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..size {
        sum += num[i].to_digit(10).expect("Err");
    } print!("{}", sum);
}