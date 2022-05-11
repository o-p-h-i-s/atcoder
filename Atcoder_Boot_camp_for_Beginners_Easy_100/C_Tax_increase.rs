use proconio::input;

fn main() {
   input! {
      a: i32,
      b: i32,
   }
   let mut ans = -1;
   for i in 1..=1e5 as i32 {
      let x = i * 8 / 100;
      let y = i * 10 / 100;
      if a == x && b == y {
         ans = i;
         break;
      }
   }
   println!("{}", ans);
}
