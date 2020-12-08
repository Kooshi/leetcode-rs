mod p0001;
mod p0002;
mod p0003;
mod p0004;
mod p0010;

fn main() {
    p0010::Solution::is_match(String::from("aab"), String::from("c*a*b"));
}
