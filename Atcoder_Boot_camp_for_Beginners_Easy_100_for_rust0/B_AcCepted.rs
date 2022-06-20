use proconio::input;

fn main() {
   input! {
      s: String,
   }
   let t = String::from(&s[2..s.len() - 1]);
   let t = t.matches('C').count();
   let n = s.chars().filter(|c| c.is_uppercase()).count();
   if s.chars().nth(0).unwrap() == 'A' && t == 1 && n == 2 {
      println!("AC");
   } else {
      println!("WA");
   }
}
