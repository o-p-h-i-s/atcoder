use proconio::input;

fn main() {
   input! {
      s: String,
   }
   let mut ans: (i32, i32) = (0, 0);
   for c in s.chars() {
      if c == '0' {
         ans.0 += 1;
      } else {
         ans.1 += 1;
      }
   }
   println!("{}", s.len() as i32 - (ans.0 - ans.1).abs());
}
