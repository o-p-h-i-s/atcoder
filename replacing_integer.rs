use proconio::input;

fn main() {
   // input
   input! {
      n: i64,
      k: i64,
   }
   // solve
   let mut ans = n % k;
   ans = std::cmp::min(ans, (ans - k).abs());
   // answer
   println!("{}", ans);
}
