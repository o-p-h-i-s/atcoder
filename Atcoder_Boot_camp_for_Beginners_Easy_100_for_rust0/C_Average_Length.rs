use itertools::Itertools;
use proconio::input;

fn main() {
   input! {
      mut n: usize,
      xy: [(f64, f64); n],
   }
   let mut ans = 0.0;
   for vec in xy.into_iter().permutations(n) {
      for i in 0..n - 1 {
         let x = (vec[i + 1].0 - vec[i].0).powf(2.0);
         let y = (vec[i + 1].1 - vec[i].1).powf(2.0);
         ans += (x + y).sqrt();
      }
   }
   for i in (1..n).rev() {
      n *= i;
   }
   println!("{}", ans / n as f64);
}
