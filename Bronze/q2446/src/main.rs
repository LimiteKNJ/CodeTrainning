fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    let mut result = String::new();

    for i in 1..num*2 {
        if i < num {
            for _ in 1..i {
                result.push(' ');
            }

            for _ in 0..2*(num-i)+1 {
                result.push('*');
            }

        } else {
            for _ in 1..num*2-i {
                result.push(' ');
            }

            for _ in 0..2*(i-num)+1 {
                result.push('*');
            }


        } result.push('\n');
    }

    print!("{}", result);
}