use proconio::input;

fn main() {
   input! {
      x: i64,
   }
   let mut ans = 0;
   for i in (1..=x).rev() {
      for j in 2..=10 {
         let n: i64 = i.pow(j);
         if 1000 < n {
            break;
         }
         if n <= x {
            ans = std::cmp::max(ans, n);
         }
      }
   }
   println!("{}", ans);
}
