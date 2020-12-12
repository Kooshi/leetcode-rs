
pub(crate) struct Solution;

enum Token {
    Char(u8),
    Star(Box<Token>),
    Dot
}

#[derive(Clone)]
enum Match {
    None,
    One(usize),
    Star(StarMatch)
}

#[derive(Clone)]
struct StarMatch {
    index:usize,
    length:usize,
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars = s.as_bytes();
        let pattern = Solution::parse_pattern(p);
        let mut matches = vec![Match::None; pattern.len()];

        let mut token_i = 0;
        let mut char_i = 0;
        loop {
            matches[token_i] = match &pattern[token_i] {
                Token::Char(c) if c == chars[char_i]  => unimplemented!(),
                Token::Star(t) => unimplemented!(),
                Token::Dot => unimplemented!(),
            }
        }

        unimplemented!()
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

    // fn matcher(t: &Token, i: &mut usize, chars: &Vec<char>) -> Match {
    //     match t {
    //         Token::Char(c) => if *i >= chars.len() || !Solution::char(c, i, chars) { return false; }
    //         Token::Dot => if *i >= chars.len() || !Solution::dot(i) { return false; }
    //         Token::Star(t) => {
    //             //TODO make less greedy
    //             while Solution::matcher(t, i, chars) {}
    //         }
    //     }
    //     return true
    // }


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