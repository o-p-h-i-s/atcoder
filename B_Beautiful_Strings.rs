use proconio::input;
use std::collections::HashMap;

fn main() {
   // input
   input! {
      w: String,
   }
   // solve
   let mut cnt = HashMap::new();
   for c in w.chars() {
      *cnt.entry(c).or_insert(0) += 1;
   }
   let mut ans = "Yes";
   for (_, val) in cnt.into_iter() {
      if val % 2 != 0 {
         ans = "No";
      }
   }
   // answer
   println!("{}", ans);
}
