use proconio::input;

fn main() {
   input! {
      n: i64,
      m: i64,
   }
   let ans = (n * m - (n * m - (n - 2) * (m - 2))).abs();
   println!("{}", ans);
}
