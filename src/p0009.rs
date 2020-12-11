
pub(crate) struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let mut larger_place = (x as f64).log10().floor() as u32;
        let mut smaller_place = 0;
        while smaller_place < larger_place {
            let smaller = (x / 10_i32.pow(smaller_place)) % 10;
            let larger = (x / 10_i32.pow(larger_place)) % 10;
            if smaller != larger{
                return false;
            }
            larger_place -= 1;
            smaller_place += 1;
        }
        true
    }
}