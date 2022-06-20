use proconio::input;

fn main() {
   input! {
      n: usize,
      t: f64,
      a: f64,
      h: [f64; n],
   }
   let mut ans = (1, (a - (t - h[0] * 0.006)).abs());
   for (i, h) in h.into_iter().enumerate() {
      let b = (a - (t - h * 0.006)).abs();
      if b <= ans.1 {
         ans = (i + 1, b);
      }
   }
   println!("{}", ans.0);
}
