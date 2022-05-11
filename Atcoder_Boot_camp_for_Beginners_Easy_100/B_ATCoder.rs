use proconio::input;

fn main() {
   // input
   input! {
      s: String
   }
   // solve
   let mut ans = 0;
   for i in 0..s.len() {
      for j in i..s.len() {
         let t = s[i..j + 1].to_string();
         let mut flag = false;
         for c in t.chars() {
            if !(c == 'A' || c == 'C' || c == 'G' || c == 'T') {
               flag = true;
            }
         }
         if flag {
            continue;
         }
         ans = std::cmp::max(ans, t.len());
      }
   }
   // answer
   println!("{}", ans);
}
