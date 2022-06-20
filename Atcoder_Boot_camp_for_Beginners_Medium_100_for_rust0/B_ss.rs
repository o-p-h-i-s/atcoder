use proconio::{input, marker::Chars};
use std::iter::FromIterator;

fn main() {
   input! {
      mut s: Chars
   }
   s.remove(s.len() - 1);
   s.remove(s.len() - 1);
   let mut ans = 0;
   while 0 < s.len() {
      let n = s.len();
      let a = String::from_iter(&s[0..n / 2]);
      let b = String::from_iter(&s[n / 2..]);
      if a == b {
         ans = n;
         break;
      }
      s.remove(s.len() - 1);
      s.remove(s.len() - 1);
   }
   println!("{}", ans);
}
