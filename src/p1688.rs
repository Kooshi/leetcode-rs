struct Solution;

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 1 {
            count+=n /2;
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = n / 2 + 1;
            }
        }
        count
    }
}