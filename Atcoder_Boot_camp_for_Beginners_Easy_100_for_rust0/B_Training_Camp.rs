use proconio::input;

fn main() {
   input! {
      n: i64,
   }
   let mut ans = 1;
   for i in 1..=n {
      ans *= i;
      ans %= 1e9 as i64 + 7;
   }
   println!("{}", ans);
}
