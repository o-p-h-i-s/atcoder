use proconio::input;
use std::collections::HashMap;

fn main() {
   // input
   input! {
      s: String,
   }
   // solve
   let mut cnt = HashMap::new();
   for c in (b'a'..b'z' + 1).map(|val| val as char) {
      *cnt.entry(c).or_insert(0) += 0;
   }
   for c in s.chars() {
      *cnt.entry(c).or_insert(0) += 1;
   }
   let mut cnt: Vec<(char, i32)> = cnt.into_iter().collect();
   cnt.sort();
   let mut ans = String::from("None");
   for (c, i) in cnt.into_iter() {
      if i == 0 {
         ans = c.to_string();
         break;
      }
   }
   // answer
   println!("{}", ans);
}
