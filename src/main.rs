use builtin_words::FINAL;
use serde_derive::{Serialize, Deserialize};
//use console;
//use std::f32::consts::E;
use colored::Colorize;
use std::io::{self, Write, BufReader, BufRead};
extern crate clap;
//use text_io::read;
use std::fs::File;
extern crate serde;
extern crate serde_json;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use clap::{ App, load_yaml};
mod builtin_words;
use crate::builtin_words::ACCEPTABLE;
mod check_word_not_tty;
use crate::check_word_not_tty::check_word_not_tty;
mod read_answer_word;
use crate::read_answer_word::read_answer_word;
use std::collections::HashMap;
use std::cmp::min;
//use serde::{Deserialize, Serialize};

mod calculate_info_entropy;
use calculate_info_entropy::cie;

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    answer : String,
    guesses : Vec<String>
}
#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    total_rounds : i64,
    games : Vec<Game>
}
// 根据答案检查用户输入的单词并为其着色 非tty模式

// 读取答案词 返回小写


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);
    

    let yml = load_yaml!("yaml.yml");

    let matches = App::from_yaml(yml).get_matches();
    


    // 处理命令行参数和config配置文件***********************************************************************
    //**************************************************************************************************
    let is_word_present: bool ; //word参数是否存在
    let mut word_value: String = String::new(); //word参数的值
    let is_random_present: bool ; //random参数是否存在
    let is_difficult_present: bool ; //difficult参数是否存在
    let is_stats_present: bool ; //stats参数是否存在
    let is_day_present: bool ; //day参数是否存在
    let mut day_value: i32 = 1; //day参数的值，默认为1
    let is_seed_present: bool ; //seed参数是否存在
    let mut seed_value: u64 = 0; //seed参数的值，默认为0
    let is_final_set_present: bool ; //final_set参数是否存在
    let mut final_set_value: String = String::new(); //final_set参数的值，默认为空
    let is_acceptable_set_present: bool ; //acceptable_set参数是否存在
    let mut acceptable_set_value: String = String::new(); //acceptable_set参数的值，默认为空
    let is_state_present: bool ; //state参数是否存在
    let mut state_value: String = String::new(); //state参数是否存在
    let is_config_present: bool = matches.is_present("config"); //config参数是否存在
    let config_value : String ; //config参数的值
    let is_screen_present : bool ;
    {
        

        if is_config_present {

            config_value = matches.value_of("config").unwrap().to_string();
            let f = File::open(config_value).unwrap();
            let v: serde_json::Value = serde_json::from_reader(f).unwrap();
            is_word_present = if matches.is_present("word") {
                word_value = matches.value_of("word").unwrap().to_string();
                matches.is_present("word")
            } else {
                match &v["word"].as_bool() {
                    None => false,
                    _ => {
                        word_value = v["word"].as_str().unwrap().to_string();
                        true
                    }
                }
            };

            is_random_present = if matches.is_present("random") {
                matches.is_present("random")
            } else {
                match &v["random"].as_bool() {
                    None => {
                        //println!("No");
                        false
                    }
                    _ => {
                        //println!("yes");
                        v["random"].as_bool().unwrap()
                    }
                }
            };

            is_difficult_present = if matches.is_present("difficult") {
                matches.is_present("difficult")
            } else {
                match &v["difficult"].as_bool() {
                    None => false,
                    _ => {
                        v["difficult"].as_bool().unwrap()
                    }
                }
            };

            is_stats_present = if matches.is_present("stats") {
                matches.is_present("stats")
            } else {
                match &v["stats"].as_bool() {
                    None => false,
                    _ => {
                        v["stats"].as_bool().unwrap()
                    }
                }
            };

            is_day_present = if matches.is_present("day") {
                day_value = matches.value_of("day").unwrap().to_string().parse().unwrap();
                matches.is_present("day")
            } else {
                match &v["day"].as_i64() {
                    None => false,
                    _ => {
                        day_value = v["day"].as_i64().unwrap().to_string().parse().unwrap();
                        true
                    }
                }
            };

            is_seed_present = if matches.is_present("seed") {
                seed_value = matches.value_of("seed").unwrap().to_string().parse().unwrap();
                matches.is_present("seed")
            } else {
                match &v["seed"].as_u64() {
                    None => false,
                    _ => {
                        seed_value = v["seed"].as_u64().unwrap().to_string().parse().unwrap();
                        true
                    }
                }
            };

            is_final_set_present = if matches.is_present("final-set") {
                final_set_value = matches.value_of("final-set").unwrap().to_string();
                matches.is_present("final-set")
            } else {
                match &v["final_set"].as_str() {
                    None => false,
                    _ => {
                        final_set_value = v["final_set"].as_str().unwrap().to_string();
                        true
                    }
                }
            };

            is_acceptable_set_present = if matches.is_present("acceptable-set") {
                acceptable_set_value = matches.value_of("acceptable-set").unwrap().to_string();
                matches.is_present("acceptable-set")
            } else {
                match &v["acceptable_set"].as_str() {
                    None => false,
                    _ => {
                        acceptable_set_value = v["acceptable_set"].as_str().unwrap().to_string();
                        true
                    }
                }
            };

            is_state_present = if matches.is_present("state") {
                matches.is_present("state")
            } else {
                match &v["state"].as_str() {
                    None => false,
                    _ => {
                        state_value = v["state"].as_str().unwrap().to_string();
                        true
                    }
                }
            };

            is_screen_present = if matches.is_present("screen") {
                matches.is_present("screen")
            } else {
                match &v["screen"].as_bool() {
                    None => {
                        //println!("No");
                        false
                    }
                    _ => {
                        //println!("yes");
                        v["screen"].as_bool().unwrap()
                    }
                }
            };

            

        } else {
            is_word_present = matches.is_present("word");
            if is_word_present {
                word_value = matches.value_of("word").unwrap().to_string();
            }
            is_random_present = matches.is_present("random");
            is_difficult_present = matches.is_present("difficult");
            is_stats_present = matches.is_present("stats");
            is_day_present = matches.is_present("day");
            if is_day_present {
                day_value = matches.value_of("day").unwrap().to_string().parse().unwrap();
            }
            is_seed_present = matches.is_present("seed");
            if is_seed_present {
                seed_value = matches.value_of("seed").unwrap().to_string().parse().unwrap();
            }
            is_final_set_present = matches.is_present("final-set");
            if is_final_set_present {
                final_set_value = matches.value_of("final-set").unwrap().to_string();
            }
            is_acceptable_set_present = matches.is_present("acceptable-set");
            if is_acceptable_set_present {
                acceptable_set_value = matches.value_of("acceptable-set").unwrap().to_string();
            }
            is_state_present = matches.is_present("state");
            if is_state_present{
                state_value = matches.value_of("state").unwrap().to_string();
            }
            is_screen_present = matches.is_present("screen");
            
        }

        if is_word_present&&is_random_present {
            Err("The - w / - word parameter is not allowed in random mode")?
        }

        if (!is_random_present)&&(is_day_present||is_seed_present) {
            Err("The - d / - day and - s / - seed parameters are not allowed in the specified answer mode")?
        }  
        
        if (!is_random_present)&&is_state_present {
            Err("The -- state parameter cannot be used in the specified answer mode")?
        }
    }


    // 获取答案词库
    let mut answer_word_list : Vec<String> = Vec::new();
    if is_final_set_present {
        let f = File::open(final_set_value).unwrap();
        let br = BufReader::new(f);
        //let mut virtual_memory = [[[[[false; 26]; 26]; 26]; 26]; 26];
        for line in br.lines() {
            let word : String = line.unwrap().trim().to_lowercase();
            if word.len() != 5 {
                Err("The final set is illegal!")?
            }
            
            for letter in word.chars() {
                if !(letter as usize >= 97 && letter as usize <= 122) {
                    Err("The final set is illegal!")?
                }
            }
            answer_word_list.push(word);
        }
    } else {
        for word in FINAL {
            answer_word_list.push(word.to_string());
        }
        //answer_word_list = FINAL;
    }
   
    // 获取可用词库
    //**************************************************************************************************
    let mut acceptable_word_list : Vec<String> = Vec::new();
    if is_acceptable_set_present {
        //let mut virtual_memory = [[[[[false; 26]; 26]; 26]; 26]; 26];
        let f = File::open(acceptable_set_value).unwrap();
        let br = BufReader::new(f);
        for line in br.lines() {
            let word : String = line.unwrap().trim().to_lowercase();
            if word.len() != 5 {
                Err("The acceptable set is illegal!")?
            }
            /*
            if acceptable_word_list.contains(&word) {
                Err("The acceptable set is illegal!")?
            }
            */

            // ---------------------------------
            /*
            for _word in &acceptable_word_list {
                if _word == &word {
                    Err("The final set is illegal!")?
                }
            }
            */
            
            // ---------------------------------
            /*
            let mut index_group: Vec<usize> = vec![0; 5];
            let mut temp_index : usize = 0;
            for letter in word.chars() {
                index_group[temp_index] = letter as usize - 97;
                temp_index += 1;
            }

            if virtual_memory[index_group[0]][index_group[1]][index_group[2]][index_group[3]][index_group[4]] {
                Err("The final set is illegal!")?
            } else {
                virtual_memory[index_group[0]][index_group[1]][index_group[2]][index_group[3]][index_group[4]] = true;
            }
            */
            
            for letter in word.chars() {
                if !(letter as usize >= 97 && letter as usize <= 122) {
                    Err("The acceptable set is illegal!")?
                }
            }
            acceptable_word_list.push(word);
        }
    } else {
        for word in ACCEPTABLE {
            acceptable_word_list.push((*word).to_string());
        }
    }

    
    // 将词库排序 检查是否为子集 检查是否有重复
    
    if is_final_set_present {
        answer_word_list.sort();
        for i in 1..answer_word_list.len() {
            if answer_word_list[i] == answer_word_list[i-1] {
                Err("The final set is illegal!")?
            }
        } 
    }
    if is_acceptable_set_present {
        acceptable_word_list.sort();
        for i in 1..acceptable_word_list.len() {
            if acceptable_word_list[i] == acceptable_word_list[i-1] {
                Err("The acceptable set is illegal!")?
            }
        }  
    }
    
    if is_acceptable_set_present || is_final_set_present {
        /*
        for i in 0..answer_word_list.len() {
            //let mut is_qualified = false;
            for j in i..acceptable_word_list.len() {
                if acceptable_word_list[j] == answer_word_list[i] {
                    //is_qualified = true;
                    break;
                }
                if acceptable_word_list[j] > answer_word_list[i] {
                    Err("The final set is not a subset of the acceptable set")?
                }
            }
        }
        */
        let mut is_sub = false;
        let mut acc_index : usize = 0;
        let mut ans_index : usize = 0;
        while acc_index < acceptable_word_list.len() {
            if answer_word_list[ans_index] == acceptable_word_list[acc_index] {
                ans_index += 1;
                if ans_index == answer_word_list.len() {
                    is_sub = true;
                    break;
                }
            }
            acc_index += 1;
        }
        if !is_sub {
            Err("The final set is not a subset of the acceptable set")?
        }
    }
    if is_day_present {
        if day_value<1||day_value>answer_word_list.len() as i32 {
            Err("Wrong --day value")?
        }
    }
    
    // 记录单词使用次数的哈希表 表中单词字母均为小写
    let mut used_words : HashMap<String, i32> = HashMap::new();


    // 游戏状态 状态中单词字母均为大写
    let mut state = GameState{ total_rounds: 0, games : Vec::new()};
    //let state_value_sub = state_value.clone();

    // 加载已有数据
    let mut total_rounds: i64 = 0; // 总局数
    let mut victory_rounds: i64 = 0; //胜利局数
    let mut victory_try_times: usize = 0; //胜利局的总尝试次数
    if is_state_present {
        // 检验文件的合法性
        let f = File::open(&state_value);
        match f {
            Ok(_) => {
                let v : Result<serde_json::Value, serde_json::Error> = serde_json::from_reader(f.unwrap());
                match &v {
                    Err(_) => Err("Illegal status file!")?,
                    _ => {
                        match &v.as_ref().unwrap()["total_rounds"].as_i64() {
                            None => {
                                // json文件为空
                                // 以上三个数据仍为 0
                            }
                            _ => {
                                // json文件不为空
                                total_rounds = v.as_ref().unwrap()["total_rounds"].as_i64().unwrap();
                                state.total_rounds = total_rounds;
                                for ob in v.as_ref().unwrap().as_object().unwrap()["games"].as_array().unwrap() {
                                    if ob.as_object().unwrap()["guesses"].as_array().unwrap().last().unwrap().as_str().unwrap() == ob.as_object().unwrap()["answer"].as_str().unwrap() {
                                        victory_rounds += 1;
                                        victory_try_times += ob.as_object().unwrap()["guesses"].as_array().unwrap().len();
                                    }
                                    let mut game = Game{answer: String::new(), guesses : Vec::new()};
                                    game.answer = ob.as_object().unwrap()["answer"].as_str().unwrap().to_string().to_uppercase();
                                    for word in ob.as_object().unwrap()["guesses"].as_array().unwrap() {
                                        game.guesses.push(word.as_str().unwrap().to_string().to_uppercase());
                                    }
                                    state.games.push(game);
                                }


                                for ob in v.as_ref().unwrap().as_object().unwrap()["games"].as_array().unwrap() {
                                    for word in ob["guesses"].as_array().unwrap() {
                                        let counts = used_words.entry(word.as_str().unwrap().to_string().to_lowercase()).or_insert(0);
                                        *counts += 1;
                                    }
                                }
                            }
                        }
                    }

                }
            }
            _ => {
                // 文件不存在
                //File::create(&state_value)?;
            }
        }
    }
    

    // 游戏开始
    let mut final_sub : Vec<usize> = Vec::new();
    let mut random_index : usize = 0;

    // 处理随机
    if is_random_present {
        let mut rand_eigine = StdRng::seed_from_u64(seed_value);
        for i in 0..answer_word_list.len() {
            final_sub.push(i);
        }
        final_sub.shuffle(&mut rand_eigine);
        random_index = (day_value - 1) as usize;
    }
    loop {
        if random_index == answer_word_list.len() {
            break;
        }
        // 开始一局游戏

        // 提高功能 记录本局游戏用过的词及其状态
        let mut word_state : Vec<(String, String)> = Vec::new();



        total_rounds += 1; // 总局数加一
        let mut is_success : bool = false; // 是否成功
        let mut trial_times = 0; // 尝试次数
            
        let mut words : Vec<String> = Vec::new(); //本局游戏用过的词
            
        // 获取答案词
        let answer_word : String;
        if is_random_present {
            answer_word = read_answer_word(is_word_present, is_random_present, &word_value, &answer_word_list, final_sub[random_index]);
        } else {
            // 输入答案词
            answer_word = read_answer_word(is_word_present, is_random_present, &word_value, &answer_word_list, 0);
            if !answer_word_list.contains(&answer_word) {
                Err("The specified answer word is not in the candidate vocabulary")?
            }
        }
        let mut alp_state = vec![-1; 26];
        let mut i : i32 = 0;
        // 开始猜词
        let mut game = Game {answer : answer_word, guesses : Vec::new()};
        let mut letter_state : Vec<String> = Vec::new();

        // 上次猜测的结果
        let mut last_chars = vec!['0'; 5];
        let mut last_color = vec!['X'; 5];

        loop {
            let mut guess_word: String = String::new();
            io::stdin()
            .read_line(&mut guess_word)
            .expect("Failed to read line");
            guess_word = guess_word.trim().to_lowercase();
            if !acceptable_word_list.contains(&guess_word.to_string()) {
                println!("INVALID");
                continue;
            }
            if is_difficult_present {
                // 困难模式
                let mut is_qulified = true;
                let mut guess_chars : Vec<char> = Vec::new();
                for i in guess_word.trim().chars() {
                    guess_chars.push(i);
                } 
                for i in 0..5 {
                    if last_color[i] == 'G' {
                        if guess_chars[i] != last_chars[i] {
                            is_qulified = false;
                            break;
                        } else {
                            guess_chars[i] = '0';
                            continue;
                        }
                    }
                }
                for i in 0..5 {
                    if last_color[i] == 'Y' {
                        let mut is_yellow_qualified = false;
                        for j in 0..5 {
                            if last_chars[i] == guess_chars[j] {
                                guess_chars[j] = '0';
                                is_yellow_qualified = true;
                                break;
                            }
                        }
                        if !is_yellow_qualified {
                            is_qulified = false;
                            break;
                        }
                    }
                }
                if !is_qulified {
                    println!("INVALID");
                    continue; 
                }
            }
            
            words.push(guess_word.clone().to_uppercase());
            let outcome = check_word_not_tty(&game.answer, &guess_word);
            // 猜测完成, 处理本次猜测数据
            

            for j in 0..26 {
                if alp_state[j] < outcome.1[j] {
                    alp_state[j] = outcome.1[j];
                }
            }
            let mut alp_state_str = String::new();
            for j in &alp_state {
                match j {
                    &-1 => alp_state_str.push('X'),
                    &0 => alp_state_str.push('R'),
                    &1 => alp_state_str.push('Y'),
                    &2 => alp_state_str.push('G'),
                    _ => unimplemented!()
                }
            }
            // 记录本次猜测
            trial_times += 1;
            let mut index = 0;
            for i in guess_word.chars() {
                last_chars[index] = i;
                index += 1;
            }
            letter_state.push(outcome.0.clone());
            let mut index = 0;
            for i in outcome.0.chars() {
                last_color[index] = i;
                index += 1;
            }
            game.guesses.push(guess_word.clone());
            if is_tty {
                for j in 0..letter_state.len() {
                    let mut state_char : Vec<char> = Vec::new();
                    let mut word_char : Vec<char> = Vec::new();
                    for c in letter_state[j].chars() {
                        state_char.push(c);
                    }
                    
                    for c in game.guesses[j].chars() {
                        word_char.push(c);
                    }
                    for k in 0..5 as usize {
                        match state_char[k] {
                            'R' => print!("{}", word_char[k].to_string().to_uppercase().red()),
                            'Y' => print!("{}", word_char[k].to_string().to_uppercase().yellow()),
                            'G' => print!("{}", word_char[k].to_string().to_uppercase().green()),
                            _ => unimplemented!()
                            
                        } 
                    }
                    println!("");
                }
                let mut all_letter : Vec<char> = Vec::new();
                for c in "abcdefghijklmnopqrstuvwxyz".chars() {
                    all_letter.push(c);
                }
                for k in 0..26 as usize {
                    match alp_state[k] {
                        -1 => print!("{}", all_letter[k].to_uppercase()),
                        0  => print!("{}", all_letter[k].to_string().to_uppercase().red()),
                        1  => print!("{}", all_letter[k].to_string().to_uppercase().yellow()),
                        2  => print!("{}", all_letter[k].to_string().to_uppercase().green()),
                        _  => unimplemented!()
                    }
                }
                println!("");
            } else {
                println!("{} {}", outcome.0, alp_state_str);
            }

            // 提高功能




            if is_screen_present {
                word_state.push((guess_word.clone(), outcome.0.clone()));
            

                let mut possible_words : Vec<String> = Vec::new();
                for word in &acceptable_word_list {
                    let mut is_qulified = true;
                    for i in 0..word_state.len() {
                        let mut guess_chars : Vec<char> = Vec::new();
                        for i in word.trim().chars() {
                            guess_chars.push(i);
                        } 

                        let mut current_chars : Vec<char> = Vec::new();
                        let mut current_color : Vec<char> = Vec::new();

                        for i in word_state[i].0.chars() {
                            current_chars.push(i);
                        }

                        for i in word_state[i].1.chars() {
                            current_color.push(i);
                        }

                        for j in 0..5 {
                            if current_color[j] == 'G' {
                                // 判断绿色是否合格
                                if guess_chars[j] != current_chars[j] {
                                    is_qulified = false;
                                    break;
                                } else {
                                    current_chars[j] = '0';
                                    guess_chars[j] = '0';
                                    
                                    continue;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("green");
                        }
                        for j in 0..5 {
                            if current_color[j] == 'Y' {
                                // 判断黄色是否合格
                                let mut is_yellow_qualified = false;


                                // 待测词与该黄色字母对应位置不是该黄色字母
                                if current_chars[j] == guess_chars[j] {
                                    is_qulified = false;
                                    break;
                                }

                                // 去除绿色后该黄色字母仍包含在待检测词中
                                for k in 0..5 {
                                    if current_chars[j] == guess_chars[k] {
                                        guess_chars[k] = '0';
                                        current_chars[j] = '0';
                                        is_yellow_qualified = true;
                                        break;
                                    }
                                }
                                
                                if !is_yellow_qualified {
                                    is_qulified = false;
                                    break;
                                }
                            } 
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("yellow");
                        }
                        for j in 0..5 {
                            //let mut is_red_qualified = true;
                            if current_color[j] == 'R' {
                                // 判断红色是否合格
                                if guess_chars.contains(&current_chars[j]) {
                                    is_qulified = false;
                                    break;
                                }
                            }
                        }
                        if !is_qulified {
                            break;
                        }else{
                            //println!("red");
                        }
                    }
                    if is_qulified {
                        possible_words.push(word.clone());
                    }
                }
                println!("{:?}", possible_words);
                let word_entropy = cie(&acceptable_word_list, &possible_words);
                for i in 0..min(5, word_entropy.len()) {
                    println!("{}  {}", word_entropy[i].0, word_entropy[i].1);
                }
            }





            








            //*********************************************************
            let counts = used_words.entry(guess_word).or_insert(0);
            *counts += 1;
            
            if &outcome.0[..] == "GGGGG" {
                is_success = true;
                if is_tty {
                    println!("CORRECT! YOU TRIED {} TIMES!", i+1);
                } else {
                    println!("CORRECT {}", i+1);
                }
                break;
            } else {
                if i == 5 {
                    if is_tty {
                        println!("FAILED! THE CORRECT WORD IS {}!", game.answer.to_uppercase());
                    } else {
                        println!("FAILED {}", game.answer.to_uppercase());
                    }
                    break;
                }
            }
            
            i += 1;
        }
        // 猜词结束，处理本局游戏的数据
        if is_success {
            victory_rounds += 1;
            victory_try_times += trial_times;
        }
        if is_state_present {
            let game = Game {answer : game.answer.to_uppercase(), guesses : words};
            state.games.push(game);
            state.total_rounds = total_rounds;
            let temp = serde_json::to_string_pretty(&state);
            match temp {
                Err(_) => Err("")?,
                _ => {}
            }
            let mut f = File::create(&state_value).unwrap();
            match f.write_all(temp.unwrap().as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    unimplemented!();
                }
            }
        }
        if is_stats_present {
            let trial_times_bar = if victory_rounds == 0 {
                0.00
            } else {
                (victory_try_times as f64)/(victory_rounds as f64)
            };
            if is_tty {
                println!("CORRECT: {}  FAILED: {}  AVERAGE ATTEMPTS: {:.2}", victory_rounds, total_rounds - victory_rounds, trial_times_bar);
            } else {
                println!("{} {} {:.2}", victory_rounds, total_rounds - victory_rounds, trial_times_bar);
            }
            
            let mut used_words_tup : Vec<(String, i32)> = Vec::new();
            let used_words_sub = used_words.clone();
            for (word, times) in used_words_sub {
                used_words_tup.push((word, times));
            }
            //println!("1");
            for i in 0..used_words_tup.len() {
                for j in 0..(used_words_tup.len()-i-1) {
                    if used_words_tup[j].1 < used_words_tup[j+1].1 {
                        let temp = used_words_tup[j].clone();
                        used_words_tup[j] = used_words_tup[j+1].clone();
                        used_words_tup[j+1] = temp;
                    } else if used_words_tup[j].1 == used_words_tup[j+1].1 {
                        if used_words_tup[j].0 > used_words_tup[j+1].0 {
                            let temp = used_words_tup[j].clone();
                            used_words_tup[j] = used_words_tup[j+1].clone();
                            used_words_tup[j+1] = temp;
                        }
                    }
                }
            }
            //println!("2");
            if is_tty {
                println!("THE MOST USED {} WORDS AND TIMES:", min(5, used_words_tup.len()));
                for i in 0..min(5, used_words_tup.len()) {
                    if i != min(5, used_words_tup.len()) - 1 {
                        println!("{} {} ", used_words_tup[i].0.to_uppercase(), used_words_tup[i].1);
                    } else {
                        println!("{} {}", used_words_tup[i].0.to_uppercase(), used_words_tup[i].1);
                    }
                
                    //println!("------")
                }
            } else {
                for i in 0..min(5, used_words_tup.len()) {
                    if i != min(5, used_words_tup.len()) - 1 {
                        print!("{} {} ", used_words_tup[i].0.to_uppercase(), used_words_tup[i].1);
                    } else {
                        print!("{} {}", used_words_tup[i].0.to_uppercase(), used_words_tup[i].1);
                    }
                }
                println!("");
            }
            
        }
        random_index += 1;
        // 询问是否继续
        if is_word_present {
            break;
        } else {
            let mut is_continue: String = String::new();
            io::stdin()
            .read_line(&mut is_continue)
            .expect("Failed to read line");
            match &is_continue.trim()[..] {
                "Y" => continue,
                "N" => break,
                _ => unimplemented!()
            }
        }
    }


















    Ok(())
}
