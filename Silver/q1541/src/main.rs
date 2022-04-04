fn main() {

    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    let mut str_sub = str.split('-').map(|s|s.trim().to_string()).collect::<Vec<String>>();
    let mut result_sum : Vec<i128> = Vec::new();
    
    for i in str_sub {
        let mut str_sum : Vec<i128> = i.split('+').map(|s|s.trim().parse().unwrap()).collect::<Vec<_>>();
        let mut result = 0 as i128;
        for j in str_sum {
            result += j;
        } result_sum.push(result);
    }

    let mut result:i128 = result_sum[0];
    for i in 1..result_sum.len() {
        result -= result_sum[i];
    }

    print!("{}", result);
}