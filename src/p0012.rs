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
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();
        while num > 999 {
            num -= 1000;
            roman.push('M');
        }

        let hun = num / 100;
        if hun == 9 {
            num -= 900;
            roman.push_str("CM")
        } else if hun >= 5 {
            num -= 500;
            roman.push('D');
        } else if hun == 4 {
            num -= 400;
            roman.push_str("CD");
        }
        while num > 99 {
            num -= 100;
            roman.push('C');
        }

        let ten = num / 10;
        if ten == 9 {
            num -= 90;
            roman.push_str("XC")
        } else if ten >= 5 {
            num -= 50;
            roman.push('L');
        } else if ten == 4 {
            num -= 40;
            roman.push_str("XL");
        }
        while num > 9 {
            num -= 10;
            roman.push('X');
        }


        if num == 9 {
            num -= 9;
            roman.push_str("IX")
        } else if num >= 5 {
            num -= 5;
            roman.push('V');
        } else if num == 4 {
            num -= 4;
            roman.push_str("IV");
        }
        while num > 0 {
            num -= 1;
            roman.push('I');
        }

        roman
    }
}