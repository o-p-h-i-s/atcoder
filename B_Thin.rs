use proconio::input;

fn main() {
   input! {
      h: usize,
      _: usize,
      c: [String; h],
   }
   for s in c.into_iter() {
      println!("{}", s);
      println!("{}", s);
   }
}
