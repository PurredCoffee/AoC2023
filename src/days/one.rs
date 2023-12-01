use std::fs::File;
use std::io::{BufReader, Read};

use super::{Day,Solution};

pub struct One {}
impl Day for One {}

impl Solution<One> for () {
  fn solve(mut f: BufReader<File>) {
    let mut buf = [0; 1];
    //initiating the variables
    // -1 will always mean that it has not changed
    let mut first = -1;
    let mut last = -1;
    //our sum variable
    let mut sum = 0;
    //all the numbers we are planning on testing
    let numbers = [
        "0".as_bytes(),
        "1".as_bytes(),
        "2".as_bytes(),
        "3".as_bytes(),
        "4".as_bytes(),
        "5".as_bytes(),
        "6".as_bytes(),
        "7".as_bytes(),
        "8".as_bytes(),
        "9".as_bytes(),
        "one".as_bytes(),
        "two".as_bytes(),
        "three".as_bytes(),
        "four".as_bytes(),
        "five".as_bytes(),
        "six".as_bytes(),
        "seven".as_bytes(),
        "eight".as_bytes(),
        "nine".as_bytes(),
    ];
    //string.toInt() cached
    //we do this because its not smart to turn "one" into 1 by doing string tests
    let to_num = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    //current index we are at for each number we are testing for (for 0-9 this will always be 0 but for unificiation we use this anyways)
    let mut indices = [0; 19];
    while f.read(&mut buf).expect("read failed") != 0 {
        //current letter
        let c = buf[0];
        //number we will set first and last to -1 if we cant set it yet
        let mut set = -1;
        //iterate through all the tested numbers
        for (i, n) in numbers.iter().enumerate() {
            //if wrong letter restart word
            if n[indices[i]] != c {
                indices[i] = 0;
            }
            //check current letter for each word
            if n[indices[i]] == c {
                //check next letter next time
                indices[i] += 1;
                //if at end of word convert to number through cache and set the "set" variable
                if indices[i] == n.len() {
                    set = to_num[i];
                    indices[i] = 0;
                }
            }
        }
        //if set changed from -1 set first and last
        if set != -1 {
            last = set;
            //skip setting first if it has already changed
            if first == -1 {
                first = last;
            }
        }
        //at end of line, last will be the last set value. aka: the last value
        if c == b'\n' {
            //if last has not changed then no number was found in this line
            if last != -1 {
                //"a" * 10 + "b" = "ab"
                sum += first * 10 + last;
                //reset first and last for next line
                first = -1;
                last = -1;
            }
        }
    }
    //because we exit the loop before we update the sum we need to manually do it outside the loop here again
    sum += first * 10 + last;
  
    //print the result!
    println!("{:?}", sum);
  }
}
