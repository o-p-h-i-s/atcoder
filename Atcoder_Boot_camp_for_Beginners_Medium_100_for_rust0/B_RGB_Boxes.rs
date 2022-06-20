use proconio::input;

fn main() {
   input! {
      r: i32,
      g: i32,
      b: i32,
      n: i32,
   }
   let mut ans = 0;
   for i in 0..=n / r {
      for j in 0..=n / g {
         let c = r * i + g * j;
         if c <= n && (n - c) % b == 0 {
            ans += 1;
         }
      }
   }
   println!("{}", ans);
}
