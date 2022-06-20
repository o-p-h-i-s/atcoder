use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      mut a: [i32; n],
   }
   // solve
   a.sort();
   a.reverse();
   let mut ans = 0;
   for (i, val) in a.into_iter().enumerate() {
      if i % 2 == 0 {
         ans += val;
      } else {
         ans -= val;
      }
   }
   // answer
   println!("{}", ans);
}
