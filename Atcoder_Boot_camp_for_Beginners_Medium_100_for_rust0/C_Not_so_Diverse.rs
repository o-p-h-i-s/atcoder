use proconio::input;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
   input! {
      n: usize,
      k: i32,
      a: [i32; n],
   }
   let mut map = HashMap::new();
   for &a in a.iter() {
      *map.entry(a).or_insert(0) += 1;
   }
   let mut a = Vec::from_iter(map.into_iter());
   a.sort_by(|a, b| a.1.cmp(&b.1));
   let mut ans = 0;
   for i in 0..a.len() as i32 - k {
      ans += a[i as usize].1;
   }
   println!("{}", ans);
}
