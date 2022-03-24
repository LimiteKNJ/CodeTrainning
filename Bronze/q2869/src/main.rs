fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let num : Vec<usize> = bf.split_whitespace()
                            .map(|s|s.trim().parse().expect("Err"))
                            .collect::<Vec<_>>();
    
    if !(num.len() < 3) {
        let result = (( num[2] - num[1] ) / ( num[0] - num[1] ));
        println!("{}", result);
    } else { panic! {"out of Bound"};}
}

// distance = up * date - down * (date - 1)
// distance - down  = ( up - down ) * date 