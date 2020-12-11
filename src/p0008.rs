
struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut is_neg = false;
        let mut chars = chars.iter().skip_while(|c| **c == b' ').peekable();
        let skip = match chars.peek() {
            Some(&&b'-') => {is_neg = true;1}
            Some(&&b'+') => 1,
            _ => 0
        };
        const INV: i32 = i32::min_value();
        let val = chars.skip(skip)
            .take_while(|c| **c >= b'0' && **c <= b'9')
            .fold(0, |a: i32, c| {
                if a == INV {
                    a
                } else {
                    a.checked_mul(10).map_or(INV, |a| a.checked_add((c - 48) as i32).unwrap_or(INV))
                }
            });

        if val == INV {
            if is_neg {
                i32::min_value()
            } else {
                i32::max_value()
            }
        } else {
            if is_neg {
                -val
            } else {
                val
            }
        }
    }
}