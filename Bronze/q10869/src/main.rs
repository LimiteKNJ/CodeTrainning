use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("no input");
    let mut num: Vec<isize> = text.split_whitespace()
                            .map(|s|s.trim().parse().expect("Error"))
                            .collect::<Vec<_>>();

    let result_p = num[0] + num[1];
    let result_m = num[0] - num[1];
    let result_mp = num[0] * num[1];
    let result_d = num[0] / num[1];
    let result_r = num[0] % num[1];
    println!("{}", result_p);
    println!("{}", result_m);
    println!("{}", result_mp);
    println!("{}", result_d);
    println!("{}", result_r);
}
