struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let bytes = n.as_bytes();
        let mut max = 0;
        for b in bytes {
            let i = b - 48;
            if i > max{
                max = i;
            }
        }
        max as i32
    }
}