
pub(crate) struct Solution;

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         if s.is_empty() {
//             return 0;
//         }
//         let mut strings = vec![String::new(); s.len()];
//         let mut mark = -1 as i32;
//         for (i,c) in s.chars().enumerate() {
//             for j in ((mark+1) as usize..i+1).rev() {
//                 if strings[j].contains(c) {
//                     mark = j as i32;
//                     break;
//                 }
//                 strings[j].push(c);
//             }
//         }
//
//         strings.iter().map(|s|s.len()).max().unwrap() as i32
//     }
// }

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         if s.is_empty() {
//             return 0;
//         }
//         let chars: Vec<char> = s.chars().collect();
//         let mut distances = vec![0 as i32; chars.len()];
//         for (i,c) in chars.iter().enumerate() {
//             for j in i+1..chars.len() {
//                 if c == &chars[j] {
//                     break;
//                 }
//                 distances[i]+=1;
//             }
//         }
//         let mut i = 0;
//         loop {
//             if i >= distances.len() {
//                 break;
//             }
//             let mut offset: usize = 1;
//             let mut min_span = distances[i];
//             loop {
//                 if offset as i32 >= min_span {
//                     break;
//                 }
//
//                 if distances[i + offset] < min_span - offset as i32 {
//                     min_span = offset as i32 + distances[i + offset];
//                     distances[i] = min_span
//                 }
//                 offset += 1;
//             }
//             i += 1;
//         }
//
//         *distances.iter().max().unwrap() + 1
//     }
// }

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut set = std::collections::HashSet::with_capacity(chars.len());
        let mut l = 0;
        let mut r = 0;
        let mut max = 0;
        while r < chars.len() {
            while r < chars.len() && set.insert(chars[r])  {
                r += 1;
                if set.len() > max {
                    max = set.len()
                }
            }
            while l < chars.len() && l < r {
                if set.remove(&chars[l]) {
                    l += 1;
                    break;
                }
                l += 1;
            }
        }

        max as i32
    }
}