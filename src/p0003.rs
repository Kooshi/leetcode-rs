use std::fs::read;

pub(crate) struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut strings = vec![String::new(); s.len()];
        let mut mark = -1 as i32;
        for (i,c) in s.chars().enumerate() {
            for j in ((mark+1) as usize..i+1).rev() {
                if strings[j].contains(c) {
                    mark = j as i32;
                    break;
                }
                strings[j].push(c);
            }
        }

        for string in strings.iter() {
            assert!(s.contains(string))
        }

        strings.iter().map(|s|s.len()).max().unwrap() as i32
    }
}