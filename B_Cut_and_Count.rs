use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: usize,
      s: String,
   }
   let mut ans = 0;
   let s = s.chars().collect::<Vec<char>>();
   for i in 0..n - 1 {
      let mut l = HashMap::new();
      let mut r = HashMap::new();
      for j in 0..n {
         if j <= i {
            *l.entry(s[j]).or_insert(0) += 1;
         } else {
            *r.entry(s[j]).or_insert(0) += 1;
         }
      }
      let mut tmp = 0;
      for (&l, _) in l.iter() {
         for (&r, _) in r.iter() {
            if l == r {
               tmp += 1;
            }
         }
      }
      ans = std::cmp::max(ans, tmp);
   }
   println!("{}", ans);
}
