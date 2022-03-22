use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Error");
    let num : Vec<usize> = text.split_whitespace()
                            .map(|s|s.trim().parse().expect("Error"))
                            .collect::<Vec<_>>();

    let check_num = (num[0] * num[0] + num[1] * num[1] + num[2] * num[2] + num[3] * num[3] + num[4] * num[4]) % 10;
    print!("{}", check_num);
}
