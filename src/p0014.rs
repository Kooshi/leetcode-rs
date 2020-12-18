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
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty(){
            return String::new();
        }
        let strs = strs.iter().map(|s|s.as_bytes()).collect::<Vec<&[u8]>>();
        let mut s = String::with_capacity(strs[0].len());
        for i in 0..strs[0].len()  {
            if !strs.iter().all(|s| i<s.len() && s[i] == strs[0][i]){
                return s;
            }
            s.push(char::from(strs[0][i]));
        }
        s
    }
}