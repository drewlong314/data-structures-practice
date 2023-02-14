pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i = 0;
    let mut shortest = strs[0].len();
    let mut answer: String = "".to_string();

    for str in &strs {
        if str.len() < shortest {
            shortest = str.len()
        }
    }

    while i < shortest {
        let mut end_loop = false;
        let prefix = strs[0].chars().take(i + 1).collect::<String>();
        for str in &strs {
            if !str.chars().take(i + 1).collect::<String>().contains(&prefix) {
                end_loop = true;
                break;
            }
        }
        if end_loop {
            break;
        }
        answer = prefix;
        i += 1;
        println!("{} end", i);
    }

    return answer;
}
