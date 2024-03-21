use std::io;

// 读取答案词 返回小写
pub fn read_answer_word(is_word_present: bool, is_random_present: bool, word_value: &String, answer: &Vec<String>, index: usize) -> String{
    if is_word_present {
        /*match word_value {
            None => { 
                unimplemented!("No word behind -w/--word");
            }
            _ => {
                matches.value_of("word").unwrap().to_string()
            }
        }
        */
        word_value.to_lowercase().trim().to_string()
    } else if is_random_present {
        answer[index].to_lowercase().trim().to_string()
        //matches.value_of("word").unwrap().to_string()
    } else {
        let mut answer_word: String = String::new();
        io::stdin()
        .read_line(&mut answer_word)
        .expect("Failed to read line");
        answer_word.trim().to_lowercase()
        //matches.value_of("word").unwrap().to_string()
    }
}