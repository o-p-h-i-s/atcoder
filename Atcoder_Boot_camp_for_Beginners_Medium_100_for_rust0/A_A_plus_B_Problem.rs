use proconio::input;

fn main() {
   input! {
      n: i64,
      a: i64,
      b: i64,
   }
   let ans = std::cmp::max(0, (b * (n - 1) + a) - (a * (n - 1) + b) + 1);
   println!("{}", ans);
}
