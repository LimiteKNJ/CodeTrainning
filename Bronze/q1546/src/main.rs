use std::io;

fn main() {
    // Input One Line
    let mut text1 = String::new();
    io::stdin().read_line(&mut text1).expect("no input");
    let num1:usize = text1.trim().parse().expect("not number");

    // Input One Line
    let mut text2 = String::new();
    io::stdin().read_line(&mut text2).expect("no input");
    let mut score_table:Vec<usize> = text2.split_whitespace()
                                            .map(|s|s.trim().parse().expect("parsing error"))
                                            .collect::<Vec<_>>();
    let mut max_score : usize = 0;

    for i in 0..num1 {
        max_score = if score_table[i] > max_score { score_table[i] } else { max_score }; 
        score_table[i] = total_score + score_table[i];
        println!("{} {}", max_score, total_score);
    }

    let result : f32 = (total_score as f32 / max_score as f32) * 100 as f32;
    println!("{}", result);
}