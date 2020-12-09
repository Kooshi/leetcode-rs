use std::iter::FromIterator;
pub(crate) struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() || s.len() == 1 {
            return s;
        }

        let chars:Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 0;
        let mut max_len = 0;
        let mut max_l = 0;
        let mut max_r = 0;
        for i in 0..chars.len()-1 {
            l = i;
            r = i+1;
            if chars[l] == chars[r] {
                Solution::span_palindrome(&chars, &mut l, &mut r);
                let len = r - l;
                if len > max_len {
                    max_len = len;
                    max_l = l;
                    max_r = r;
                }
            }

            l = i;
            r = i+1;
            if l > 0 && chars[l-1] == chars[r] {
                l -= 1;
                Solution::span_palindrome(&chars, &mut l, &mut r);
                let len = r - l;
                if len > max_len {
                    max_len = len;
                    max_l = l;
                    max_r = r;
                }
            }
        }
        String::from_iter(&chars[max_l..max_r+1])
    }

    fn span_palindrome(chars:&Vec<char>, l:&mut usize, r:&mut usize) {
        loop {
            if *l == 0 || *r == chars.len() - 1 {
                return;
            }

            *l -= 1;
            *r += 1;

            if chars[*l] != chars[*r] {
                *l+=1;
                *r-=1;
                return;
            }
        }
    }
}