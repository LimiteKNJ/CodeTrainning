use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.trim().parse::<usize>().unwrap();
    let mut vec : VecDeque<usize> = VecDeque::new();

    for i in 0..num {
        vec.push_front(i+1);
    }

    let mut result = 0;
    while !vec.is_empty() {
        if vec.len() == 1 {
            result = vec.pop_back().unwrap();
            break;
        }

        vec.pop_back().unwrap();
        let temp = vec.pop_back().unwrap();
        vec.push_front(temp);
        
    } print!("{}", result);
}