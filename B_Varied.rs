use proconio::input;

fn main() {
   // input
   input! {
      mut s: String,
   }
   // solve
   let mut s: Vec<char> = s.chars().collect();
   s.sort();
   let mut ans = "yes";
   let mut tmp = s[1];
   for c in s.into_iter() {
      if tmp == c {
         ans = "no";
         break;
      }
      tmp = c;
   }
   // answer
   println!("{}", ans);
}
