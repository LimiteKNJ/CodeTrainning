use std::io;
use io::Write;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut ipv6_zip = buf.trim().chars().collect::<Vec<char>>(); ipv6_zip.push('\n');
    buf.clear();

    let mut ipv6 = String::new();
    let mut ipv6_part = String::new();

    let mut iter = 0;
    let mut ipv6_end_null = false;
    let mut ipv6_zero_index = 0;
    let mut ipv6_cnt = 0;
    let mut char_cnt = 0;

    for i in ipv6_zip {
        
        if i.is_alphanumeric() {
            ipv6_part.push(i);
            char_cnt += 1;
        }

        if (i.eq(&':') || i.eq(&'\n')) && !ipv6_part.is_empty() {
            if ipv6_part.len() >= 4 {
                ipv6.push_str(&ipv6_part);
                ipv6_part.clear();

                if i.eq(&':') {
                    ipv6.push(':');
                } char_cnt = 0;

            } else {
                let mut ipv6_temp = String::new();
                char_cnt = 4 - char_cnt;
                while char_cnt > 0 {
                    ipv6_temp.push('0');
                    char_cnt -= 1;
                    iter += 1;
                } ipv6_temp.push_str(&ipv6_part);
                ipv6.push_str(&ipv6_temp);
                ipv6_part.clear();

                if i.eq(&':') {
                    ipv6.push(':');
                }
            } ipv6_cnt += 1;
        }

        else if (i.eq(&':') || i.eq(&'\n')) && ipv6_part.is_empty() {
            if i.eq(&':') && ipv6_cnt != 0 {
                ipv6_zero_index = iter;

            } else if i.eq(&'\n') && ipv6_cnt != 0 {
                ipv6_end_null = true;

            } else if i.eq(&':') && ipv6_cnt == 0 {
                ipv6_zero_index = 0;

            } else if i.eq(&'\n') && ipv6_cnt == 0 {
                ipv6_zero_index = 0; ipv6_end_null = true;
            }
        } iter += 1;
    }

    if ipv6_cnt < 8 {

        for _ in 0..8-ipv6_cnt {
            ipv6.insert_str(ipv6_zero_index,"0000:");
        }

        if ipv6_end_null {
            ipv6.pop();
        }
    }
    write!(out, "{}", ipv6).unwrap();
}