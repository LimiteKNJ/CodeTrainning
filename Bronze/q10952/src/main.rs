use std::io;

fn main() {
    let mut result : Vec<usize> = vec![];
    let mut count : usize = 0;

    loop {
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("no input");
        let mut num: Vec<usize> = text.split_whitespace()
                                .map(|s|s.trim().parse().expect("Error"))
                                .collect::<Vec<_>>();

        if num[0] == 0 && num[1] == 0 {
            break;
        } else {
            result.push(num[0] + num[1]);
            count += 1;
        }
    }

    for _i in 0..count{
        println!("{}", result[_i]);
    }
}