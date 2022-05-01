use proconio::input;

fn main() {
   // input
   input! {
      _: usize,
      s: String,
   }
   // solve
   let s: Vec<char> = s.chars().collect();
   let mut ans = 0;
   let mut tmp = 0;
   for c in s.into_iter() {
      tmp += if c == 'I' { 1 } else { -1 };
      ans = std::cmp::max(ans, tmp);
   }
   // answer
   println!("{}", ans);
}
