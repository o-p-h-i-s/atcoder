use proconio::input;

fn main() {
   // input
   input! {
      n: i64,
      a: i64,
      b: i64,
   }
   // solve
   let mut ans = (n / (a + b)) * a;
   ans += std::cmp::min(a, n % (a + b));
   // answer
   println!("{}", ans);
}
