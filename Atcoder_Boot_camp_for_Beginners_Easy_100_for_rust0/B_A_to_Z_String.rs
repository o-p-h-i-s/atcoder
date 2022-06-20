use proconio::input;

fn main() {
   input! {
      s: String,
   }
   let n = (s.len() - s.chars().rev().position(|c| c == 'Z').unwrap()) as i32
      - s.chars().position(|c| c == 'A').unwrap() as i32;
   println!("{}", n);
}
