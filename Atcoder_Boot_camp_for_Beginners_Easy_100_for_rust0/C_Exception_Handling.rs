use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      a: [i32; n],
   }
   // solve
   let mut b = a.clone();
   b.sort();
   // answer
   for val in a.into_iter() {
      if val != b[n - 1] {
         println!("{}", b[n - 1]);
      } else {
         println!("{}", b[n - 2]);
      }
   }
}
