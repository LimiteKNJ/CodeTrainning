fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let infix = bf.trim().to_uppercase().chars().collect::<Vec<char>>();
    let mut postfix : Vec<char> = Vec::new();

//  a*b+(c+d) = ab*cd++
    let mut into_p = false;
    let mut op : Vec<char> = Vec::new();

    for i in 0..infix.len(){
        if infix[i].is_alphabetic() {
            postfix.push(infix[i]);
        } else if infix[i] == '+' || infix[i] == '-' {
            if into_p {
                postfix.push(op.pop().expect("Err"));
            } op.push(infix[i]);
                
        } else if infix[i] == '*' || infix[i] == '/' {
            op.push(infix[i]);
        } else if infix[i] == '(' {
            into_p = true;

        } else if infix[i] == ')' {
            if into_p {
                postfix.push(op.pop().expect("Err"));
                into_p = false;
            } else {
                print!("err"); break;
            }
        } else {
            print!("err");
        }
    }

    for _ in 0..op.len(){
        postfix.push(op.pop().expect("Err"));
    }

    print!("{:?}", postfix);
}
