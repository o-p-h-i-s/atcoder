use proconio::{input, marker::Chars};

fn main() {
   input! {
      s: Chars,
   }
   let mut i = 0;
   let mut ans = 0;
   let mut prev = String::from("?");
   while i < s.len() {
      if s[i].to_string() == prev {
         i += 1;
         if i == s.len() {
            break;
         }
         prev = prev + &s[i].to_string();
      } else {
         prev = s[i].to_string();
      }
      i += 1;
      ans += 1;
   }
   println!("{}", ans);
}
