use proconio::input;

fn main() {
   input! {
      s: String,
      t: String,
   }
   let s = s.repeat(2);
   if s.contains(&t) {
      println!("Yes");
   } else {
      println!("No");
   }
}
