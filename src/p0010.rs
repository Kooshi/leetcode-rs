

pub(crate) struct Solution;

enum Token {
    Char(char),
    Star(Box<Token>),
    Dot
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let pattern = Solution::parse_pattern(p);
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        for t in pattern {
            if !Solution::matcher(&t, &mut i, &chars) {
                return false;
            }
        }
        if i == chars.len() {
            true
        } else {
            false
        }
    }

    fn char(c: &char, i: &mut usize, chars: &Vec<char>) -> bool {
        if chars[*i] == *c {
            *i += 1;
            true
        } else {
            false
        }
    }

    fn dot(i: &mut usize) -> bool {
        *i += 1;
        true
    }

    fn matcher(t: &Token, i: &mut usize, chars: &Vec<char>) -> bool {
        match t {
            Token::Char(c) => if *i >= chars.len() || !Solution::char(c, i, chars) { return false; }
            Token::Dot => if *i >= chars.len() || !Solution::dot(i) { return false; }
            Token::Star(t) => {
                //TODO make less greedy
                while Solution::matcher(t, i, chars) {}
            }
        }
        return true;
    }


    fn parse_pattern<'a>(p: String) -> Vec<Token> {
        let mut pattern: Vec<Token> = Vec::with_capacity(p.len());

        for c in p.chars() {
            if c == '.' {
                pattern.push(Token::Dot);
            } else if c == '*' {
                let star = Token::Star(Box::new(pattern.pop().unwrap()));
                pattern.push(star);
            } else {
                pattern.push(Token::Char(c));
            }
        }

        pattern
    }
}