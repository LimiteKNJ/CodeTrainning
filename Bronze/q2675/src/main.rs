fn main() {
    
    let mut resultV : Vec<String> = Vec::new();
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let times : usize = bf.trim().parse().expect("Err");

    for i in 0..times {
        
        let mut bf2 = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut bf2).expect("Err");
        let txt : Vec<String> = bf2.split_whitespace()
                                    .map(|s|s.trim().parse().expect("Err"))
                                    .collect::<Vec<_>>();

        if !(txt.len() < 2) {
            let cnt : usize = txt[0].parse().expect("Err");
            let temp = txt[1].chars().collect::<Vec<_>>();
            let mut result = String::new();

            for i in 0..temp.len() {
                for j in 0..cnt {
                    result.push(temp[i]);
                }
            } resultV.push(result);

        } else { continue; }
    }

    for k in 0..resultV.len() {
        println!("{}", resultV[k]);
    }
}
