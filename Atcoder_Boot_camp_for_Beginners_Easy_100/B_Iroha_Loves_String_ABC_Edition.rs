use proconio::input;

fn main() {
   input! {
      n: usize,
      _: usize,
      mut s: [String; n],
   }
   s.sort();
   println!("{}", s.into_iter().collect::<Vec<_>>().join(""));
}
