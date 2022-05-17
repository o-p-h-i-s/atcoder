use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: i32,
      a: [i32; n],
   }
   let mut map = HashMap::new();
   for &a in a.iter() {
      *map.entry(a).or_insert(0) += 1;
   }
   let mut ans = 0;
   for (_, x) in map.into_iter() {
      if x % 2 == 1 {
         ans += 1;
      }
   }
   println!("{}", ans);
}
