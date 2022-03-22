fn main() {
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let bf_rev = bf.chars().rev().collect::<String>();
    let num : Vec<usize> = bf_rev.split_whitespace()
                            .map(|s|s.trim().parse().expect("Err"))
                            .collect::<Vec<_>>();

    if !(num.len() < 2) {
        if num[0] >= num[1] {
            print!("{}", num[0]);
        } else { print!("{}", num[1]); }
    } else { panic!("Out of Bound Err");}
}
