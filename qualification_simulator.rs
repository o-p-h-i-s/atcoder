use proconio::input;

fn main() {
   // input
   input! {
      _: usize,
      a: usize,
      b: usize,
      s: String,
   }
   // solve
   // answer
   let mut ent = 0;
   let mut fent = 1;
   for c in s.chars() {
      let mut flag = true;
      match c {
         'a' => {
            if ent < a + b {
               ent += 1;
            } else {
               flag = false;
            }
         }
         'b' => {
            if ent < a + b && fent <= b {
               ent += 1;
               fent += 1;
            } else {
               flag = false
            }
         }
         _ => flag = false,
      }
      println!("{}", if flag { "Yes" } else { "No" });
   }
}
