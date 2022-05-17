use proconio::input;

fn main() {
   input! {
      n: i64,
   }
   let mut ans = 0;
   for i in 1..=n {
      ans += i % n;
   }
   println!("{}", ans);
}
