use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
   }
   let vec = vec![1, 2, 4, 8, 16, 32, 64];
   // solve
   let mut ans = 1;
   for val in vec {
      if val <= n {
         ans = val;
      } else {
         break;
      }
   }
   // answer
   println!("{}", ans);
}
