pub fn longest_common_prefix(strs: Vec<String>) -> String {

    // convert the first string into an array of chars or return an empty string
    let mut answer: Vec<char> = match strs.first() {
        Some(s) => s.chars().collect(),
        None => return String::new(),
    };

    for str in strs {
        let mut new_answer:Vec<char> = Vec::new();

        // covert the string into an array of chars with indexes
        for (i, char) in str.chars().enumerate() {
            // check if the length of the first string is greater than the index to catch out of index errors
            // check if the current char is equal to the char located at the index of the previous answer
            if answer.len() > i && char == answer[i] {
                new_answer.push(char);
            } else {
                break;
            }
        }

        answer = new_answer;
    }

    let mut final_answer = String::new();
    // convert the array of chars into a string
    answer.iter().for_each(|chr| final_answer.push(chr.to_owned()));
    return final_answer;


    // let mut i = 0;
    // let mut shortest = strs[0].len();
    // let mut answer: String = "".to_string();

    // for str in &strs {
    //     if str.len() < shortest {
    //         shortest = str.len()
    //     }
    // }

    // while i < shortest {
    //     let mut end_loop = false;
    //     let prefix = strs[0].chars().take(i + 1).collect::<String>();
    //     for str in &strs {
    //         if !str.chars().take(i + 1).collect::<String>().contains(&prefix) {
    //             end_loop = true;
    //             break;
    //         }
    //     }
    //     if end_loop {
    //         break;
    //     }
    //     answer = prefix;
    //     i += 1;
    // }

    // return answer;
}
