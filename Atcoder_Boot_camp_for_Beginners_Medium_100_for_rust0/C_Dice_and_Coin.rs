use proconio::input;

fn main() {
   input! {
      n: f64,
      k: f64
   }
   let mut ans = 0.0;
   let mut l = 1.0;
   while l <= n {
      let mut point = l;
      let mut cnt = 0.0;
      while point < k {
         point *= 2.0;
         cnt += 1.0;
      }
      ans += 1.0 / (n * 2f64.powf(cnt));
      l += 1.0;
   }
   println!("{}", ans);
}
