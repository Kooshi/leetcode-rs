
pub(crate) struct Solution;

enum Token {
    Char(char),
    Star(Box<Token>),
    Dot
}

impl Token {
    fn consumes(&self, c:&char) -> bool {
        match self {
            Token::Char(tc) => *tc == *c,
            Token::Star(t) => t.consumes(c),
            Token::Dot => true
        }
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let chars:Vec<char> = s.chars().collect();
        let pattern = Solution::parse_pattern(p);

        Solution::matches(pattern.as_slice(), chars.as_slice())
    }


    fn matches(pattern: &[Token], chars: &[char]) -> bool {
        for (i,t) in pattern.iter().enumerate() {
            if let Token::Star(t) = t {
                let mut ii = i;
                if Solution::matches(&pattern[i + 1..], &chars[ii..]){
                    return true;
                }
                while ii < chars.len() && t.consumes(&chars[ii]) {
                    if Solution::matches(&pattern[i + 1..], &chars[ii+1..]){
                        return true;
                    }
                    ii+=1;
                }
                return false;
            } else if i>= chars.len() || !t.consumes(&chars[i]) {
                return false;
            }
        }
        pattern.len() == chars.len()
    }


    fn parse_pattern(p: String) -> Vec<Token> {
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