use std::{fs::File, io::{BufRead, BufReader}};

pub fn solve1() -> i32 {
  let mut s: i32 = 0;
  let mut fst_digit: i32;
  let mut snd_digit: i32;
  let input = File::open("../input1.txt").unwrap();
  // create a buffer to read the file line by line
  let mut buf_reader = BufReader::<File>::new(input);
  let number_words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

  // create a loop to read the file line by line
  let mut line: String = String::new();
  while let Ok(_) = buf_reader.read_line(&mut line) {
    if line.is_empty() {
      break;
    };
    // find the first numeric character and extract it into fst_digit
    let first_digit_position = line.find(|c: char| c.is_numeric()).unwrap();
    fst_digit = line[first_digit_position..first_digit_position+1].parse::<i32>().unwrap();
    // find the last numeric character
    let second_digit_position = line.rfind(|c: char| c.is_numeric()).unwrap();
    snd_digit = line[second_digit_position..second_digit_position+1].parse::<i32>().unwrap();
  
    let mut first_word_position = line.len();
    let mut first_word_length = 0;
    // search for any word in the number_words array from the left
    for word in number_words.iter() {
      if line.contains(word) {
        let word_position = line.find(word).unwrap();
        if word_position < first_word_position {
          first_word_position = word_position;
          first_word_length = word.len();
        }
      }
    }
    if first_word_position < first_digit_position {
      // get index of the word at first_word_position from the number_words array
      fst_digit = number_words.iter().position(|&r| r == &line[first_word_position..first_word_position+first_word_length]).unwrap() as i32;
    }
    
    let mut last_word_position = 0;
    let mut last_word_length = 0;
    // search for any word in the number_words array from the right
    for word in number_words.iter() {
      if line.contains(word) {
        let word_position = line.rfind(word).unwrap();
        if word_position > last_word_position {
          last_word_position = word_position;
          last_word_length = word.len();
        }
      }
    }
    if last_word_position > second_digit_position {
      // get index of the word at last_word_position from the number_words array
      snd_digit = number_words.iter().position(|&r| r == &line[last_word_position..last_word_position+last_word_length]).unwrap() as i32;
    }

    let n = 10 * fst_digit + snd_digit;
    print!("fst:{} snd:{} n:{} line: {}", fst_digit, snd_digit, n, line);
    s += n;
    
    // clear the line
    line.clear();
      
  }

  s

}
