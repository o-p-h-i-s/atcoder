use proconio::input;

fn main() {
   // input
   input! {
      h: i64,
      w: i64,
   }
   // solve
   // answer
   if h == 1 || w == 1 {
      println!("1");
   } else {
      println!("{}", (h * w + 1) / 2);
   }
}
