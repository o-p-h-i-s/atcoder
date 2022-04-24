use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      x: [i32; n],
   }
   // solve
   let mut tmp1 = 0;
   let mut tmp2 = 0;
   for &i in x.iter() {
      tmp1 += i * i;
      tmp2 += i;
   }
   let mut ans = std::i32::MAX;
   for p in 0..100 {
      let tmp3 = tmp1 + p * p * n as i32 - 2 * p * tmp2;
      ans = std::cmp::min(ans, tmp3);
   }
   // answer
   println!("{}", ans);
}
