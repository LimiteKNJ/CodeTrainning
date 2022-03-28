fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let case = bf.trim().parse::<usize>().expect("Err");

    for _ in 0..case {

        let mut bf = String::new();
        std::io::stdin().read_line(&mut bf).expect("Err");
        let ptext : Vec<char> = bf.trim().chars().collect();
        let mut p : Vec<char> = Vec::new();
        let mut err = false;

        for i in 0..ptext.len() {
            if ptext[i] == '(' {
                p.push(ptext[i]);
            } else if ptext[i] == ')' {
                if !p.is_empty() {
                   p.pop();
                } else {
                   err = true; break;
                }
            } else {
                print!("Error"); break;
            }
        }

        if p.is_empty() && !err {
            println!("YES");
        } else { println!("NO"); }
    }
}