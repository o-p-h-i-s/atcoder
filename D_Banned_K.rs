use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: usize,
      a: [i64; n],
   }
   let mut map = HashMap::new();
   for &a in a.iter() {
      *map.entry(a).or_insert(0) += 1i64;
   }
   let mut sum = 0;
   for (_, &v) in map.iter() {
      sum += v * (v - 1) / 2;
   }
   for &a in a.iter() {
      let v = *map.get(&a).unwrap();
      let ans = sum - v + 1;
      println!("{}", ans);
   }
}
