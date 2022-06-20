use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: usize,
      s: [String; n],
   }
   let mut t = HashMap::new();
   for s in s.into_iter() {
      *t.entry(s).or_insert(0) += 1;
   }
   let mut s = t.into_iter().collect_vec();
   s.sort_by(|a, b| b.1.cmp(&a.1));
   let maxs = s[0].1;
   let mut t = Vec::new();
   for (k, v) in s.into_iter() {
      if v == maxs {
         t.push(k);
      }
   }
   t.sort();
   for t in t.into_iter() {
      println!("{}", t);
   }
}
