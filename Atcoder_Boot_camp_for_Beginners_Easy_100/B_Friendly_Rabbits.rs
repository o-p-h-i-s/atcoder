use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [usize; n],
   }
   let mut ans = 0;
   for i in 0..n {
      if a[a[i] - 1] == i + 1 {
         ans += 1
      }
   }
   println!("{}", ans / 2);
}
