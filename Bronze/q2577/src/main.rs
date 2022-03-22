fn main() {

    let mut v_num : Vec<usize> = Vec::new();
    for i in 0..3 {
        let mut bf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut bf).expect("Err");
        v_num.push(bf.trim().parse().expect("Err"));
    }

//    println!("{}", v_num[0] * v_num[1] * v_num[2]);
    let result : Vec<char> = (v_num[0] * v_num[1] * v_num[2]).to_string().chars().collect::<Vec<_>>();
    let mut count = 0;

    for i in 0..10{
        for j in 0..result.len() {
            if result[j] as usize == i + '0' as usize {
                count += 1;
            }
        } println!("{}", count);
        count = 0;
    }
}
