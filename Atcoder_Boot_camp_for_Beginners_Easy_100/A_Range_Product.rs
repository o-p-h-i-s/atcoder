use proconio::input;

fn main() {
   // input
   input! {
      a: i64,
      b: i64,
   }
   // solve
   let ans;
   if (0 < a && 0 < b) || (a < 0 && b < 0 && (b - a) % 2 != 0) {
      ans = String::from("Positive");
   } else if a <= 0 && 0 <= b {
      ans = String::from("Zero");
   } else {
      ans = String::from("Negative");
   }
   // answer
   println!("{}", ans);
}
