use proconio::input;

fn main() {
   input! {
      s: String,
      t: String,
   }
   let mut s = s.chars().collect::<Vec<char>>();
   let mut t = t.chars().collect::<Vec<char>>();
   s.sort();
   t.sort();
   t.reverse();
   println!("{}", if s < t { "Yes" } else { "No" });
}
