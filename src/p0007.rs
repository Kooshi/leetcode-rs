
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::min_value() || x == i32::max_value() {
            return 0;
        }

        let is_neg = x.is_negative();
        let mut x = x.abs();
        let mut y: i32 = 0;
        while x != 0 {
            let (ry, over) = y.overflowing_mul(10);
            if over {
                return 0;
            }
            let (ry, over) = ry.overflowing_add(x % 10);
            if over {
                return 0;
            }
            y = ry;
            x /= 10;
        }

        if is_neg {
            -y
        } else {
            y
        }
    }
}