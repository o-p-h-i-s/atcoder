use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut count = HashMap::new();
   for &a in a.iter() {
      *count.entry(a + 1).or_insert(0) += 1;
      *count.entry(a).or_insert(0) += 1;
      *count.entry(a - 1).or_insert(0) += 1;
   }
   println!("{}", count.values().max().unwrap());
}
