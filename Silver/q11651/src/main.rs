fn main() {
    let mut pointV : Vec<(isize, isize)> = Vec::new();

    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let point_cnt : usize = bf.trim().parse().expect("Err");

    for _ in 0..point_cnt {
        let mut bf2 = String::new();
        stdin.read_line(&mut bf2).expect("Err");
        let temp : Vec<isize> = bf2.split_whitespace()
                                .map(|s|s.trim().parse().expect("Err"))
                                .collect::<Vec<_>>();
        if !(temp.len() < 2) {
            pointV.push((temp[0], temp[1])) 
        }
    }

    pointV.sort_by(|(a,b),(c,d)|(b, a).cmp(&(d, c)));

    let mut str = String::new();
    for i in 0..pointV.len() {
        let (x, y) = pointV[i];
        str.push_str(&format!("{} {}\n", x, y).to_string());
    } println!("{}", str);
}