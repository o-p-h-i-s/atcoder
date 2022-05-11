use proconio::input;
use std::collections::HashMap;

fn main() {
   // input
   input! {
      n: usize,
      _: usize,
   }
   // solve
   let mut ans = 0;
   let mut cnt = HashMap::new();
   for _ in 0..n {
      input! {
         k: usize,
         a: [i32; k],
      }
      for val in a.into_iter() {
         *cnt.entry(val).or_insert(0) += 1;
      }
   }
   for (_, val) in cnt.into_iter() {
      if val == n {
         ans += 1;
      }
   }
   // answer
   println!("{}", ans);
}
