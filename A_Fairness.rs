use proconio::input;

fn main() {
   // input
   input! {
      mut a: i128,
      mut b: i128,
      _: i128,
      k: i64,
   }
   // solve
   let ans = if k % 2 == 0 { a - b } else { b - a };
   // answer
   println!("{}", ans);
}
