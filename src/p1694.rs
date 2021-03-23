
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number: Vec<char> = number.replace(" ", "").replace("-", "").chars().collect();
        let last = match number.len() % 3 {
            0 => 0,
            1 => 4,
            2 => 2,
            _ => panic!()
        };

        let mut result = String::with_capacity(number.len() + number.len() / 3);
        let mut i = 0;
        while i < number.len() - last {
            result.push(number[i]);
            result.push(number[i + 1]);
            result.push(number[i + 2]);
            result.push('-');
            i += 3;
        }
        if last == 0 {
            result.pop();
        } else if last == 4 {
            result.push(number[i]);
            result.push(number[i + 1]);
            result.push('-');
            result.push(number[i + 2]);
            result.push(number[i + 3]);
        } else {
            result.push(number[i]);
            result.push(number[i + 1]);
        }
        result
    }
}