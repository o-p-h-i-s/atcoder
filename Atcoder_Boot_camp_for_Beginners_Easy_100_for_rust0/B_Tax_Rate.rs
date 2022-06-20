use proconio::input;

fn main() {
   // input
   input! {
      n: f64,
   }
   // solve
   let mut ans = std::i32::MAX;
   for i in 1..50000 {
      let x = i as f64 * 1.08;
      if x.floor() == n {
         ans = i;
         break;
      }
   }
   // answer
   if ans == std::i32::MAX {
      println!(":(");
   } else {
      println!("{}", ans);
   }
}
