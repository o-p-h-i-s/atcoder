use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      k: i32,
      x: [i32; n],
   }
   // solve
   let mut ans = 0;
   for a in x.into_iter() {
      let b = (a - k).abs();
      ans += std::cmp::min(a, b) * 2;
   }
   // answer
   println!("{}", ans);
}
