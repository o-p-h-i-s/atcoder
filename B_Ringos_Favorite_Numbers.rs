use proconio::input;

fn main() {
   input! {
      d: u32,
      n: i32,
   }
   let ans;
   if n == 100 {
      ans = 101 * 100i32.pow(d);
   } else {
      ans = n * 100i32.pow(d);
   }
   println!("{}", ans);
}
