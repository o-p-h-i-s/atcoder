use proconio::input;

fn main() {
   // input
   input! {
      a: i32,
      mut b: i32,
   }
   // solve
   let ans = (b - 1 + a - 2) / (a - 1);
   // answer
   println!("{}", ans);
}
