pub fn check_word_not_tty(answer_word: &String, guess_word: &String) -> (String, Vec<i32>) {
    //let answer_word_sub: String = answer_word.clone();
    //let guess_word_sub: String = guess_word.clone();
    let mut alp_state = vec![-1; 26];
    let mut answer_chars: Vec<char> = Vec::new();
    for i in answer_word.chars() {
        answer_chars.push(i);
    }
    let mut guess_chars: Vec<char> = Vec::new();
    for i in guess_word.chars() {
        guess_chars.push(i);
    }

    //let answer_chars_sub  = answer_chars.clone();
    let guess_chars_sub  = guess_chars.clone();
    let mut output = vec!['0'; 5];
    for i in 0..5 {
        if &answer_chars[i] == &guess_chars[i] {
            output[i] = 'G';
            answer_chars[i] = '0';
            guess_chars[i] = '0';
        }
    }
    for i in 0..5 {
        if &guess_chars[i] != &'0' {
            for j in 0..5 {
                if &guess_chars[i] == &answer_chars[j] {
                    output[i] = 'Y';
                    guess_chars[i] = '0';
                    answer_chars[j] = '0';
                    break;
                }
            }
        }
    }
    for i in 0..5 {
        if &output[i] == &'0' {
            output[i] = 'R';
        }
    }

    for i in 0..5 {
        match &output[i] {
            &'G' => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = 2;
            }
            &'Y' => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = if alp_state[(guess_chars_sub[i] as usize) - 97] < 1 {
                    1
                } else {
                    alp_state[(guess_chars_sub[i] as usize) - 97]
                }
            }
            &'R' => {
                alp_state[(guess_chars_sub[i] as usize) - 97] = if alp_state[(guess_chars_sub[i] as usize) - 97] < 0 {
                    0
                } else {
                    alp_state[(guess_chars_sub[i] as usize) - 97]
                }
            }
            _ => unimplemented!()
        }
    }
    (output.iter().collect::<String>(), alp_state)

}