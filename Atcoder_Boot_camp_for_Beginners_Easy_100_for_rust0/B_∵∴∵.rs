use proconio::input;

fn main() {
   input! {
      o: String,
      e: String,
   }
   let mut pass = o
      .chars()
      .zip(e.chars())
      .map(|(a, b)| a.to_string() + &b.to_string())
      .collect::<String>();
   if e.len() < o.len() {
      pass.push(o.chars().last().unwrap());
   }
   println!("{}", pass);
}
