use proconio::input;

fn main() {
   input! {
      mut x: i64,
      y: i64,
   }
   let mut ans = 1;
   while x * 2 <= y {
      x *= 2;
      ans += 1;
   }
   println!("{}", ans);
}
