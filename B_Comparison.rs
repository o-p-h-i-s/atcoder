use proconio::input;

fn main() {
   // input
   input! {
      a: String,
      b: String,
   }
   // solve
   let ans = if b.len() < a.len() {
      "GREATER"
   } else if a.len() < b.len() {
      "LESS"
   } else {
      let mut tmp = "EQUAL";
      for (a, b) in a.chars().zip(b.chars()) {
         if a == b {
            continue;
         } else if b < a {
            tmp = "GREATER";
            break;
         } else {
            tmp = "LESS";
            break;
         }
      }
      tmp
   };
   // answer
   println!("{}", ans);
}
