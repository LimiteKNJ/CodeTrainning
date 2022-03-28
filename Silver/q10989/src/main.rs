use std::io::*;

fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let cnt = bf.trim().parse::<usize>().expect("Err");

    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let mut num_vec = vec![0 as usize ; 10001];
    for _ in 0..cnt {
        let mut bf2 = String::new();
        std::io::stdin().read_line(&mut bf2).expect("Err");
        let num = bf2.trim().parse::<usize>().expect("Err");

        if num <= 10000 && num >= 1 {
            num_vec[num] += 1;
        } else { println!("Err"); }
    }

    for i in 1..num_vec.len() {
        if num_vec[i] == 0 {
            continue;
        } else {
            for _ in 0..num_vec[i] {
                write!(out, "{}\n", &i.to_string()).expect("Err");
            }
        }
    }
}