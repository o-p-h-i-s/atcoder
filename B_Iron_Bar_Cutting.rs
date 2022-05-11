use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i64; n],
   }
   let mut l = 0;
   let mut r = a.iter().sum::<i64>();
   let mut ans = std::i64::MAX;
   for a in a.into_iter() {
      l += a;
      r -= a;
      ans = std::cmp::min(ans, (l - r).abs());
   }
   println!("{}", ans);
}
