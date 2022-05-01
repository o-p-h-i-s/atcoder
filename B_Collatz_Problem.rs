use proconio::input;
use std::collections::HashMap;

fn main() {
   // input
   input! {
      mut s: i32,
   }
   // solve
   let mut collatz = HashMap::new();
   let mut cnt = 1;
   let ans;
   loop {
      if let Some(_) = collatz.get_mut(&s) {
         ans = cnt;
         break;
      } else {
         collatz.insert(s, cnt);
      }
      cnt += 1;
      s = if s % 2 == 0 { s / 2 } else { 3 * s + 1 };
   }
   // answer
   println!("{}", ans);
}
