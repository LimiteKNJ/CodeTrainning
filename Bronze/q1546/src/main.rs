use std::io;

fn main() {
    // Input One Line
    let mut text1 = String::new();
    io::stdin().read_line(&mut text1).expect("no input");
    let num1:usize = text1.trim().parse().expect("not number");

    // Input One Line
    let mut text2 = String::new();
    io::stdin().read_line(&mut text2).expect("no input");
    let mut score_table:Vec<f32> = text2.split_whitespace()
                                            .map(|s|s.trim().parse().expect("parsing error"))
                                            .collect::<Vec<_>>();
    let mut max_score : f32 = 0f32;
    let mut total_score : f32 = 0f32;

    for i in 0..num1 {
        max_score = if score_table[i] > max_score { score_table[i] } else { max_score };
    }

    for i in 0..num1 {
        score_table[i] = ( score_table[i] / max_score ) * 100f32;
        total_score = total_score + score_table[i];
    }

    let result = total_score / num1 as f32;
    println!("{}", result);
}