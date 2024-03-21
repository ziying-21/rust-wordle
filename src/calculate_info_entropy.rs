pub fn cie(total_acc : &Vec<String>, maybe : &Vec<String>) -> Vec<(String, f64)> {
    

    let mut word_entropy : Vec<(String, f64)> = Vec::new();

    for word_1 in total_acc {
        let mut counts = [[[[[0; 3]; 3]; 3]; 3]; 3];
        for word_2 in maybe {
            let mut word_1_chars : Vec<char> = Vec::new();
            let mut word_2_chars : Vec<char> = Vec::new();
            let mut outcome = vec![0; 5];
            for i in word_1.chars() {
                word_1_chars.push(i);
            }
            for i in word_2.chars() {
                word_2_chars.push(i);
            }
            for i in 0..5 {
                if &word_1_chars[i] == &word_2_chars[i] {
                    outcome[i] = 2;
                    word_1_chars[i] = '0';
                    word_2_chars[i] = '0';
                }
            }
            for i in 0..5 {
                if &word_1_chars[i] != &'0' {
                    for j in 0..5 {
                        if &word_1_chars[i] == &word_2_chars[j] {
                            outcome[i] = 1;
                            word_1_chars[i] = '0';
                            word_2_chars[j] = '0';
                            break;
                        }
                    }
                }
            }
            counts[outcome[0]][outcome[1]][outcome[2]][outcome[3]][outcome[4]] += 1;
        }

        let mut info_entropy = 0.0;
        for i in counts {
            for j in i {
                for k in j {
                    for l in k {
                        for m in l {
                            if m == 0 {
                                continue;
                            }
                            info_entropy += -(m as f64/maybe.len() as f64)*(m as f64/maybe.len() as f64).log2();
                        }
                    }
                }
            }
        }
        word_entropy.push((word_1.clone(), info_entropy));
    }
    for i in 0..word_entropy.len() {
        for j in 0..word_entropy.len() - i - 1 {
            if word_entropy[j].1 < word_entropy[j+1].1{
                let temp = word_entropy[j].clone();
                word_entropy[j] = word_entropy[j+1].clone();
                word_entropy[j+1] = temp;
            }
        }
    }
    word_entropy

}