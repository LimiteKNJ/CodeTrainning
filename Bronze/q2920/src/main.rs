use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Err");
    let scale : Vec<usize> = text.split_whitespace()
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<_>>();

    let mut correct_cnt = 0;
    let mut reverse_cnt = 0;
    for i in 0..8 {
        if scale[i] == i + 1 {
            correct_cnt = correct_cnt + 1;
        }
        if scale[i] == 8 - i {
            reverse_cnt = reverse_cnt + 1;
        }
    }

    if correct_cnt == 8 { print!("ascending"); }
    else if reverse_cnt == 8 { print!("descending"); }
    else { print!("mixed"); }
}