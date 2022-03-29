const NULL : usize = 0;
const OPEN_HTML : usize = 101;
const CLOSE_HTML : usize = 102;
const DIV_OPEN : usize = 111;
const DIV_CLOSE : usize = 112;
const P_OPEN : usize = 121;
const P_CLOSE : usize = 122;
const B_OPEN : usize = 131;
const B_CLOSE : usize = 132;
const ITALIC_OPEN : usize = 141;
const ITALIC_CLOSE : usize = 142;
const BR_FLAGS : usize = 150;

fn html_tag_check (tags : String) -> usize {

    let mut flags = NULL;
    if tags == "main" { flags = OPEN_HTML; }
    if tags == "/main" { flags = CLOSE_HTML; }
    if tags == "div title" { flags = DIV_OPEN; }
    if tags == "/div"{ flags = DIV_CLOSE; }
    if tags == "p" { flags = P_OPEN; }
    if tags == "/p" { flags = P_CLOSE; }
    if tags == "b" { flags = B_OPEN; }
    if tags == "/b"{ flags = B_CLOSE; }
    if tags == "i" { flags = ITALIC_OPEN; }
    if tags == "/i"{ flags = ITALIC_CLOSE; }
    if tags == "br "{ flags = BR_FLAGS; }

    return flags;
}



fn main() {
    let mut bf = String::new();
    std::io::stdin().read_line(&mut bf).expect("Err");
    let mut str = bf.trim().chars().collect::<Vec<char>>();
    str.push('\n');

    let mut temp = String::new();
    let mut iter = 0;

    let mut p_open = false;
    let mut c_open = false;
    let mut err = false;

    let mut html = false;
    let mut div = false;
    let mut p = false;
    let mut bold = false;
    let mut italic = false;
    let mut trg = false;

    let mut start_p = false;
    let mut end_p = false;
    let mut is_more_whitespace = false;

    let mut stack : Vec<usize> = Vec::new();
    let mut result = String::new();

    while str[iter] != '\n' {

        if !p_open && str[iter] == '<' {
            p_open = true;
        
        } else if p_open && str[iter] == '>' {
            let tags = temp.clone();
            let flags = html_tag_check(tags);
            if flags != NULL {
                stack.push(flags);
            } temp.clear();
            trg = true;
            p_open = false;

        } else if !p_open && str[iter] == '>' {

            println!("> NOT CLOSED.");
            err = true; break;
        
        } else if p_open && (str[iter].is_alphanumeric() || str[iter].is_whitespace() ||
                        str[iter] == '/' || str[iter] == '=' || str[iter] == '\"' || str[iter] == '_') {

            // Excption <Div>
            if str[iter] == '=' {
                result.push_str("title : ");

            } else if !c_open && str[iter] == '\"' {
                c_open = true;

            } else if c_open && str[iter] == '\"' {
                result.push('\n');
                c_open = false;

            } else if c_open && (str[iter].is_alphanumeric() || str[iter].is_whitespace() || str[iter] == '_') {
                result.push(str[iter]);

            // Else
            } else {
                temp.push(str[iter].to_ascii_lowercase());
            }
        
        } else if !p_open && html && div && p && str[iter] != '>' {

            if str[iter].is_whitespace() && !start_p && !is_more_whitespace {
                result.push(str[iter]);
                is_more_whitespace = true;
                end_p = false;

            } else if str[iter].is_whitespace() && ( start_p || end_p || is_more_whitespace ){
            
            } else {
                result.push(str[iter]);
                start_p = false;
                end_p = true;
                is_more_whitespace = false;
            }
        }

        // HTML TAG

        if !stack.is_empty() && trg == true {

            match stack[stack.len()-1] {

                OPEN_HTML => html = true,

                CLOSE_HTML => {
                    stack.pop(); stack.pop(); html = false;
                    if !stack.is_empty() {
                        println!("CAN NOT CLOSE HTML."); err = true;
                        break;
                    }
                }

                DIV_OPEN => {
                    div = true;
                }
                DIV_CLOSE => {
                    stack.pop(); stack.pop(); div = false;
                }
                P_OPEN => {
                    start_p = true; p = true;
                }
                P_CLOSE => {
                    stack.pop(); stack.pop();
                    result.push('\n'); p = false;
                }
                B_OPEN => {
                    if italic == false {
                        bold = true;
                    } else { println!("Not Closed Another Flags"); err = true; break; }
                }
                B_CLOSE => {
                    if bold == true {
                        stack.pop(); stack.pop(); bold = false;
                    } else { println!("Not Closed Current Flags"); err = true; break; }
                }
                ITALIC_OPEN => {
                    if bold == false {
                        italic = true;
                    } else { println!("Not Closed Another Flags"); err = true; break; }
                }
                ITALIC_CLOSE => {
                    if italic == true {
                        stack.pop(); stack.pop(); italic = false;
                    } else { println!("Not Closed Current Flags"); err = true; break; }
                }
                BR_FLAGS => {
                    stack.pop();
                }
                _ => { println!("Unknown tags"); err = true; break; }
                
            } trg = false;
        }

        iter += 1;
    }

    if !err {
        println!("{}", result);
    }
}
