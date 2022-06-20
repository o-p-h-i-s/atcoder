use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      p: [i32; n],
   }
   // solve
   let mut ans = 0;
   let mut l = std::i32::MAX;
   for val in p.into_iter() {
      if val < l {
         l = val;
         ans += 1;
      }
   }
   // answer
   println!("{}", ans);
}
