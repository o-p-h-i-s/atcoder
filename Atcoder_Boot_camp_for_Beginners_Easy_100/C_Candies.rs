use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [[i32; n]; 2],
   }
   let mut b = vec![vec![0; n + 1]; 2];
   for i in 0..n {
      b[0][i + 1] = b[0][i] + a[0][i];
      b[1][n - i - 1] = b[1][n - i] + a[1][n - i - 1];
   }
   let mut ans = 0;
   for i in 0..n {
      let tmp = b[0][i + 1] + b[1][i];
      ans = std::cmp::max(ans, tmp);
   }
   println!("{}", ans);
}
