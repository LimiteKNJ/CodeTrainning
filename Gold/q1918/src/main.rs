fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let infix = bf.trim().to_uppercase().chars().collect::<Vec<char>>();
    let mut postfix : Vec<char> = Vec::new();

//  a*b+(c+d) = ab*cd++
    let mut op : Vec<char> = Vec::new();

    for i in 0..infix.len(){
        if infix[i].is_alphabetic() {
            postfix.push(infix[i]);

        } else if infix[i] == '(' {
            op.push('|');

        } else if infix[i] == '*' || infix[i] == '/' {
            if !op.is_empty() {
                if op[op.len()-1] == '*' || op[op.len()-1] == '/' {
                    postfix.push(op.pop().expect("Err"));
                }
            } op.push(infix[i]);

        } else if infix[i] == '+' || infix[i] == '-' {
            while !op.is_empty() && op[op.len()-1] != '|' {
                postfix.push(op.pop().expect("Err"));
            } op.push(infix[i]);
            
        } else if infix[i] == ')' {
            while op[op.len()-1] != '|' {
                postfix.push(op.pop().expect("Err"))
            } op.pop().expect("Err");

        } else {
            print!("err");
        }
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