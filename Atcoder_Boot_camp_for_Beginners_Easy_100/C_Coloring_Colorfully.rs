use proconio::input;

fn main() {
   // input
   input! {
      w: String,
   }
   // solve
   let mut ans = std::i32::MAX;
   for i in 0..=1 {
      let mut cnt = 0;
      let mut cmp = i;
      for c in w.chars() {
         if c.to_digit(10).unwrap() == cmp {
            cnt += 1;
         }
         cmp ^= 1;
      }
      ans = std::cmp::min(ans, cnt);
   }
   // answer
   println!("{}", ans);
}
