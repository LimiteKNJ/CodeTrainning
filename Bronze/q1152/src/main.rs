fn main() {
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let words : Vec<String> = bf.split_whitespace()
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<_>>();
    print!("{}", words.len());
}