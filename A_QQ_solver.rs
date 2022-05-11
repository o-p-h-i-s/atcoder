use proconio::input;

fn main() {
   input! {
      s: String
   }
   let s = s.chars().collect::<Vec<char>>();
   let ans = (s[0] as i32 - '0' as i32) * (s[2] as i32 - '0' as i32);
   println!("{}", ans);
}
