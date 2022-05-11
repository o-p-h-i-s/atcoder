use proconio::input;

fn main() {
   input! {
      n: usize,
      k: usize,
   }
   println!("{}", k * (k - 1).pow(n as u32 - 1));
}
