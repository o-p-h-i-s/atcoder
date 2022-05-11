use proconio::input;

fn main() {
   input! {
      s: String,
      k: i64,
   }
   let mut ans = '1';
   for (i, c) in s.chars().enumerate() {
      if c != '1' {
         ans = c;
         break;
      } else if k - 1 == i as i64 {
         break;
      }
   }
   println!("{}", ans);
}
