use proconio::input;
use std::collections::HashSet;

fn main() {
   input! {
      a: i32,
      b: i32,
      k: i32,
   }
   let mut ans = Vec::new();
   for i in 0..k {
      let x = a + i;
      let y = b - i;
      if a <= x && x <= b {
         ans.push(x)
      }
      if a <= y && y <= b {
         ans.push(y)
      }
   }
   let ans: HashSet<i32> = ans.into_iter().collect();
   let mut ans: Vec<i32> = ans.into_iter().collect();
   ans.sort();
   for ans in ans.into_iter() {
      println!("{}", ans);
   }
}
