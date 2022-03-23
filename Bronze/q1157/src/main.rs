fn main() {

    let alpha = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut alpha_cnt : [usize; 26] = [0; 26]; // alpha cnt
    let mut max = 0;

    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let spelling : Vec<char> = bf.to_uppercase().trim().chars().collect::<Vec<_>>();

    for i in 0..alpha.len() {
        for j in 0..spelling.len(){
            if spelling[j] as usize == i + 'A' as usize {
                alpha_cnt[i] += 1;
            }
        }

        if alpha_cnt[i] > max {
            max = alpha_cnt[i];
        }
    }

    let mut char = '0';
    for i in 0..alpha.len() {
        if alpha_cnt[i] == max && char == '0'{
            char = alpha[i];
        } else if alpha_cnt[i] == max && char != '0' {
            char = '?';
        }
    }

    print!("{}", char );
}
