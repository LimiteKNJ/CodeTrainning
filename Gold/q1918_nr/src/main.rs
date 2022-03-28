fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let infix = bf.trim().to_uppercase().chars().collect::<Vec<char>>();
    let mut postfix : Vec<char> = Vec::new();

//  a*b+(c+d) = ab*cd++
    let mut op : Vec<char> = Vec::new();
    let mut p_open = 0;

    for i in 0..infix.len(){
        if infix[i].is_alphabetic() {
            postfix.push(infix[i]);

        } else if infix[i] == '(' {
            op.push('|');
            p_open += 1;

        } else if infix[i] == '*' || infix[i] == '/' {
            op.push(infix[i]);

        } else if infix[i] == '+' || infix[i] == '-' {
            if p_open == 0 {
                while !op.is_empty() {
                    postfix.push(op.pop().expect("Err"));
                } op.push(infix[i]);
            } else {
                op.push(infix[i]);
            }
            
        } else if infix[i] == ')' {
            if p_open >= 1 {
                while op[op.len()-1] != '|' {
                    postfix.push(op.pop().expect("Err"))
                } op.pop().expect("Err"); p_open -= 1;
            } else { println!("Not Open!") }

        } else {
            print!("err");
        }

        println!("{:?}", op);
    }

    for _ in 0..op.len(){
        postfix.push(op.pop().expect("Err"));
    }

    let mut result = String::new();
    for i in 0..postfix.len(){
        result.push(postfix[i]);
    }
    print!("{}", result);
}