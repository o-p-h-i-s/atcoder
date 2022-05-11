use proconio::input;

fn main() {
   input! {
      n: usize,
      mut m: i64,
      mut ab: [(i64, i64); n],
   }
   ab.sort();
   let mut ans = 0;
   for &(a, b) in ab.iter() {
      if b < m {
         ans += a * b;
         m -= b;
      } else {
         ans += a * m;
         break;
      }
   }
   println!("{}", ans);
}
