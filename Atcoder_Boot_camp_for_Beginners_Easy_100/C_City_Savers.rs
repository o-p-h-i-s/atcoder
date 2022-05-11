use proconio::input;

fn main() {
   input! {
      n: usize,
      mut a: [i64; n + 1],
      b: [i64; n],
   }
   let mut ans = 0;
   for i in 0..n {
      if a[i] <= b[i] {
         ans += a[i];
         if b[i] - a[i] <= a[i + 1] {
            ans += b[i] - a[i];
            a[i + 1] -= b[i] - a[i];
         } else {
            ans += a[i + 1];
            a[i + 1] = 0;
         }
      } else {
         ans += b[i];
      }
   }
   println!("{}", ans);
}
