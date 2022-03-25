fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let var_str : Vec<char> = bf.trim().chars().collect();
    let mut result = String::new();

    let mut java_cnt = 0; let mut cpp_cnt = 0;
    let mut cpp_trg = 0;

    if !(var_str.len() < 1) {
        if var_str[0] == '_' || var_str[0].is_uppercase() {
            print!("Error!");
        } else {
            result.push(var_str[0]);
            for i in 1..var_str.len() {
                if var_str[i] == '_' {
                    cpp_trg += 1;
                    cpp_cnt += 1;
                } else if var_str[i].is_uppercase() {
                    result.push('_');
                    result.push(var_str[i].to_ascii_lowercase());
                    java_cnt += 1;
                } else {
                    if cpp_trg == 1 {
                        result.push(var_str[i].to_ascii_uppercase());
                        cpp_trg -= 1;
                    } else { result.push(var_str[i]); }
                }
            }
        }
    } else { panic!("Out of Bound")}

    if cpp_cnt != 0 && java_cnt != 0 {
        print!("Error!");
    } else if cpp_trg != 0 {
        print!("Error!");
    } else {
        print!("{}", result);
    }
}