use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i64; n],
   }
   let mut even = 0;
   for &a in a.iter() {
      if a % 2 == 0 {
         even += 1;
      }
   }
   println!("{}", 3i64.pow(n as u32) - 2i64.pow(even));
}
