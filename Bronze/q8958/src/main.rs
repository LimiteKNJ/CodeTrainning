fn main() {
    
    let mut scoreV:Vec<usize> = Vec::new();

    let mut text = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut text).expect("Err");
    let times : usize = text.trim().parse().expect("Err");

    for i in 0..times {

        let mut combo = 0;
        let mut score = 0;
        
        let mut question = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut question).expect("Err");

        for c in question.chars() {
            if c == 'O' {
                combo += 1;
                score += combo;
            } else {
                combo = 0;
            }
        }

        scoreV.push(score);
    }

    for j in 0..times {
        println!("{}", scoreV[j]);
    }
}
