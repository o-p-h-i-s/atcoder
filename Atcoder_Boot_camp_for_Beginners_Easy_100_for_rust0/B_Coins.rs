use proconio::input;

fn main() {
   // input
   input! {
      a: usize,
      b: usize,
      c: usize,
      x: usize,
   }
   // solve
   let mut ans = 0;
   for i in 0..a + 1 {
      for j in 0..b + 1 {
         for k in 0..c + 1 {
            let tmp = 500 * i + 100 * j + 50 * k;
            if tmp == x {
               ans += 1;
            }
         }
      }
   }
   // answer
   println!("{}", ans);
}
