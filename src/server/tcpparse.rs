pub fn parse_command_and_argument(to_parse: String) -> Vec<String> {
    let iter = to_parse.split("\n").collect::<Vec<&str>>();
    let mut parsed: Vec<String> = Vec::new();
    for i in iter {
        if !i.contains("\u{0}") {
            let split = i.split(" ").map(|e| e.trim()).collect::<Vec<&str>>();

            for i in split.clone() {
                parsed.push(format!("{}", i));
            }
        }
    }
    parsed
}

pub fn get_argument_from_parsed(parsed: Vec<String>) -> Vec<String> {
    let mut n = 0;
    let mut result: Vec<String> = Vec::new();
    for i in parsed {
        if n == 0 {
            n += 1;
            continue;
        } else {
            result.push(i);
        }
    }
    result
}

pub fn get_argument_from_parsed_to_string(parsed: Vec<String>) -> String {
    let mut n = 0;
    let mut result: String = String::new();
    for i in parsed.clone() {
        if n == 0 {
            n += 1;
            continue;
        } else if n == parsed.len() - 1 {
            result.push_str(format!("{}", i).as_str());
        } else {
            result.push_str(format!("{} ", i).as_str());
        }
        n += 1;
    }
    result
}
