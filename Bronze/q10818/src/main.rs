use std::io;

fn main() {
    let mut text1 = String::new();
    io::stdin().read_line(&mut text1).expect("no input");
    let size:usize = text1.trim().parse().expect("not number");
                        
    let mut text2 = String::new();
    io::stdin().read_line(&mut text2).expect("no input");
    let mut num:Vec<isize> = text2.split_whitespace()
                            .map(|s|s.trim().parse().expect("parsing error"))
                            .collect::<Vec<_>>();

    let mut max = -1000000;
    let mut min = 1000000;
    for i in 0..size {
        if max < num[i] {max = num[i];}
        if min > num[i] {min = num[i];}
    }

    print!("{} {}", min, max);
}