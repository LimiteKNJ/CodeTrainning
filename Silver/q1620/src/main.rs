use std::collections::HashMap;
use std::io;
use io::Write;

/*
fn find_key_for_value<'a>(map: &'a HashMap<usize, String>, value: String) -> Option<&'a usize> {
    map.iter()
        .find_map(|(key, val)| if val == &value { Some(key) } else { None })
}
 */

fn main() {
    let mut buf = String::new();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    std::io::stdin().read_line(&mut buf).unwrap();
    let num = buf.split_whitespace()
                        .map(|s|s.trim().parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
    
    let dic_num = num[0];
    let ques_num = num[1]; 
    let mut dic : HashMap<usize, String> = HashMap::new();
    let mut dic_rs : HashMap<String, usize> = HashMap::new();

    for iter in 1..=dic_num {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let name = str.trim();
        dic.insert(iter, name.to_string());
        dic_rs.insert(name.to_string(), iter);
    }

    let mut result = String::new();
    for _ in 0..ques_num {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        let answer = str.trim().to_string();

        if answer.parse::<usize>().is_ok() {
            let num = answer.parse::<usize>().unwrap();
            result.push_str(dic.get(&num).unwrap());
            result.push('\n');
            
        } else {
            let name = answer;
            result.push_str(&dic_rs.get(&name).unwrap().to_string());
            result.push('\n');
        }
    } write!(out, "{}", result).unwrap();
}