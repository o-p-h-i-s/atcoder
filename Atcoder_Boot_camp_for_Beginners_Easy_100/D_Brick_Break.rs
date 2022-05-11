use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut ans = -1;
   let mut j = 1;
   for i in 0..n {
      if a[i] == j {
         ans = n as i32 - j;
         j += 1;
      }
   }
   println!("{}", ans);
}
