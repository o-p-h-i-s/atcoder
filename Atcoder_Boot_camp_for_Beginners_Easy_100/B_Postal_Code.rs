use proconio::input;

fn main() {
   // input
   input! {
      a: usize,
      _: i32,
      s: String,
   }
   // solve
   let s: Vec<char> = s.chars().collect();
   let mut ans = true;
   for i in 0..s.len() {
      if i == a {
         if s[i] != '-' {
            ans = false;
            break;
         }
      } else {
         if s[i] == '-' {
            ans = false;
            break;
         }
      }
   }
   // answer
   println!("{}", if ans { "Yes" } else { "No" });
}
