use proconio::input;

fn main() {
   // input
   input! {
      s: String,
   }
   // solve
   let mut ans = std::i32::MAX;
   for i in 0..s.len() - 2 {
      let x = &s[i..i + 3];
      let x: i32 = x.parse().unwrap();
      ans = std::cmp::min(ans, (753 - x).abs());
   }
   // answer
   println!("{}", ans);
}
