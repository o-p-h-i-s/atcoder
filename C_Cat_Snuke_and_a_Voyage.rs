use proconio::input;
use std::collections::HashSet;

fn main() {
   input! {
      n: i32,
      m: usize,
      ab: [(i32, i32); m],
   }
   let mut v = HashSet::new();
   let mut w = HashSet::new();
   for &(a, b) in ab.iter() {
      if a == 1 {
         v.insert(b);
      }
      if b == n {
         w.insert(a);
      }
   }
   let mut ans = "IMPOSSIBLE";
   for v in v.iter() {
      if w.contains(v) {
         ans = "POSSIBLE";
         break;
      }
   }
   println!("{}", ans);
}
