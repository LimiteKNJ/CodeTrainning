fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let num = bf.split(',')
                            .map(|s|s.trim().parse().expect("Err"))
                            .collect::<Vec<usize>>();
    let mut sum = 0;
    for i in 0..num.len() {
        sum += num[i];
    } print!("{}", sum);
}