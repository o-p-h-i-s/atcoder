use proconio::input;

fn main() {
   input! {
      a: i32,
      b: i32,
      c: i32,
   }
   let mut bool = false;
   for i in 1..b {
      if (a * i - c) % b == 0 {
         bool = true;
         break;
      }
   }
   println!("{}", if bool { "YES" } else { "NO" });
}
