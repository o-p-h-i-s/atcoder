use proconio::input;

fn main() {
   // input
   input! {
      a: String,
      b: String,
   }
   // solve
   let num = a + b.as_str();
   let num: f64 = num.parse().unwrap();
   // answer
   println!(
      "{}",
      if num == (num.sqrt().round()).powf(2.0) {
         "Yes"
      } else {
         "No"
      }
   );
}
