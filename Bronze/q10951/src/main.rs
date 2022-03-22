fn main() {
    let mut result : Vec<usize> = Vec::new();
    let mut count = 0;

    loop {
        let mut text = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut text).expect("Err");
        let num : Vec<usize> = text.split_whitespace()
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<_>>();
        if !(num.len() < 2) {
            result.push(num[0] + num[1]);
            count += 1;
        } else { break; }
    }

    for i in 0..count {
        println!("{}", result[i]);
    }
}