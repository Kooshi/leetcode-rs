#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(true,true)
    }

    // use test::Bencher;
    // #[bench]
    // fn bench(b: &mut Bencher) {
    //     b.iter(|| )
    // }
}
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let numerals = ['I', 'V', 'X', 'L', 'C', 'D', 'M', ' ', ' '];
        let mut i = 6;
        let mut int = 0;
        let mut add = 1000;
        for c in s.chars() {
            loop {
                int += match c {
                    k if k == numerals[i] => add,
                    k if k == numerals[i + 1] => 3 * add,
                    k if k == numerals[i + 2] => 8 * add,
                    k if k == numerals[i - 1] => add / 2,
                    _ => {
                        i -= 2;
                        add /= 10;
                        continue;
                    }
                };
                break;
            }
        }
        int
    }
}