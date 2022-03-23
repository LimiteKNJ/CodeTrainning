fn main() {
    let mut numV : Vec<usize> = Vec::new();
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let times : usize = bf.trim().parse().expect("Err");

    for _ in 0..times {
        let mut bf = String::new();
        stdin.read_line(&mut bf).expect("Err");
        let num : usize = bf.trim().parse().expect("Err");

        if num != 0 {
            numV.push(num);
        } else {
            numV.pop().expect("Out of Bound");
        }
    }

    let mut sum = 0;
    for i in 0..numV.len() {
        sum += numV[i];
    } println!("{}",sum);
}