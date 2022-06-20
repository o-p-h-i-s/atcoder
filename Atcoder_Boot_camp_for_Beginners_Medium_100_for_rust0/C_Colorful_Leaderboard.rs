use proconio::input;
use std::collections::HashSet;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut b = HashSet::new();
   let mut c = 0;
   for &a in a.iter() {
      let n = a / 400;
      if 8 <= n {
         c += 1;
      } else {
         b.insert(n);
      }
   }
   let ans = (std::cmp::max(1, b.len()), b.len() + c);
   println!("{} {}", ans.0, ans.1);
}
