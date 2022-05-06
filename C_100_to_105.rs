use proconio::input;

fn main() {
   input! {
      x: i32,
   }
   let n = x / 100;
   let m = (x % 100 + 4) / 5;
   if m <= n {
      println!("1");
   } else {
      println!("0");
   }
}
