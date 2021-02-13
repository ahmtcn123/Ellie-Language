pub mod terminal_colors;

pub struct ReliableNameRangeResponse {
    pub reliable: bool,
    pub at: usize,
    pub found: char
}

pub enum ReliableNameRanges {
    VariableName,
    Type,
    FunctionName
}

pub fn is_opearators(value: &str) -> bool {
    let operators = "|&";
    operators.contains(&value)
}

pub fn reliable_name_range(_range: ReliableNameRanges, value: String) -> ReliableNameRangeResponse {
    let variable_range = "qwertyuıopasdfghjklizxcvbnm0123456789";

    
    let find = value.split("").position(|x| !variable_range.contains(&x));
    return ReliableNameRangeResponse {
        reliable: find == None,
        at: find.unwrap_or(0),
        found: value.chars().nth(if let Some(e) = find {e - 1} else {0}).unwrap_or_default()
    };
}

pub fn get_letter(letter: String, index: usize, turn: bool) -> String {
    if turn { // Bir sonraki karakter
        if index == letter.len() {
            return "".to_string();
        } else {
            let sliced: Vec<char> = letter.chars().skip(index + 1).take(1).collect();
            return if sliced.len() == 0 { "".to_string() } else { sliced[0].to_string()};
        }
    } else {
        if index == 0 || index == 1 {
            return "".to_string();
        } else {
            let sliced: Vec<char> = letter.chars().skip(index - 1).take(1).collect();
            return if sliced.len() == 0 { "".to_string() } else { sliced[0].to_string()};
        }
    }
}

pub fn get_line(code: String, line: usize) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    v[line].to_string()
}

pub fn arrow(line: usize, range: usize) -> String {
    let mut s = String::with_capacity(line);
    let mut range_arrows = String::with_capacity(range);
    for _ in 0..range {range_arrows.push_str("^")};
    if line == 0 {
        s = range_arrows;
    } else {
        for e in 0..line {
            if e == line - 1 {
                s.push_str(&range_arrows);
            } else {
                s.push_str(" ");
            }
        }
    }
    return s;
}