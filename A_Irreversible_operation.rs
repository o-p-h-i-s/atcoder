use proconio::{input, marker::Chars};

fn main() {
   input! {
      s: Chars
   }
   let mut ans = 0;
   let mut j = 0;
   let n = s.len();
   for i in 0..n {
      if s[n - 1 - i] == 'B' {
         ans += i - j;
         j += 1;
      }
   }
   println!("{}", ans);
}
