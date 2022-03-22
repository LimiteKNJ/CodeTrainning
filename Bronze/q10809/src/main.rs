fn main() {

    let mut alpa:[isize; 26]= [-1; 26];
    let mut bf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut bf).expect("Err");
    let txt : Vec<char> = bf.chars().collect::<Vec<_>>();

    for i in 0..alpa.len(){
        let mut cnt = false;
        for j in 0..txt.len() {
            if txt[j] as isize == 'a' as isize + i as isize && cnt == false {
                alpa[i] = j as isize;
                cnt = true;
            }
        } cnt = false;
    }

    for j in 0..alpa.len() {
        print!("{} ",alpa[j]);
    }
}
