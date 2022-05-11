use proconio::input;

fn main() {
   input! {
      n: usize,
      k: usize,
      mut h: [i32; n],
   }
   h.sort();
   let mut ans = std::i32::MAX;
   for i in 0..=n - k {
      ans = std::cmp::min(ans, h[i + k - 1] - h[i])
   }
   println!("{}", ans);
}
