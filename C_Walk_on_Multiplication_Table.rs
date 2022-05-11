use proconio::input;

fn main() {
   input! {
      n: i64,
   }
   let mut ans = std::i64::MAX;
   let mut i = 1;
   while i * i <= n {
      if n % i == 0 {
         ans = std::cmp::min(ans, (i - 1) + (n / i - 1));
      }
      i += 1;
   }
   println!("{}", ans);
}
