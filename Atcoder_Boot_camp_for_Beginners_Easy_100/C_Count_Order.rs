use itertools::Itertools;
use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      p: [usize; n],
      q: [usize; n],
   }
   // solve
   let perm: Vec<Vec<usize>> = (1..=n).permutations(n).collect();
   let p = perm.iter().position(|vec| *vec == p).unwrap() as i32;
   let q = perm.iter().position(|vec| *vec == q).unwrap() as i32;
   let ans = (p - q).abs();
   // answer
   println!("{}", ans);
}
