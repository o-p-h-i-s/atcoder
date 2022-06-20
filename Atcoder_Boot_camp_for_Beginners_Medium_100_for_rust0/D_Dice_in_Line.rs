use proconio::input;

fn main() {
   input! {
      n: usize,
      k: usize,
      p: [f64; n],
   }
   let q = p
      .iter()
      .map(|a| a * (a + 1.0) / (2.0 * a))
      .collect::<Vec<f64>>();
   let mut sum = vec![0.0; n + 1];
   for i in 0..n {
      sum[i + 1] = q[i] + sum[i];
   }
   let mut ans = 0.0;
   for i in 0..n - k + 1 {
      let tmp = sum[i + k] - sum[i];
      ans = if ans <= tmp { tmp } else { ans };
   }
   println!("{:.10}", ans);
}
