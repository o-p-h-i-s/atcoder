use proconio::input;

fn main() {
   input! {
      s: String,
   }
   let mut ans = "Yes";
   if s.contains('N') {
      if !s.contains('S') {
         ans = "No";
      }
   }
   if s.contains('S') {
      if !s.contains('N') {
         ans = "No";
      }
   }
   if s.contains('W') {
      if !s.contains('E') {
         ans = "No";
      }
   }
   if s.contains('E') {
      if !s.contains('W') {
         ans = "No";
      }
   }
   println!("{}", ans);
}
