use proconio::input;

fn main() {
   input! {
      n: usize,
      b: [i32; n - 1],
   }
   let mut a = vec![std::i32::MAX; n];
   for i in 0..n - 1 {
      a[i] = std::cmp::min(a[i], b[i]);
      a[i + 1] = std::cmp::min(a[i + 1], b[i]);
   }
   let ans = a.iter().sum::<i32>();
   println!("{}", ans);
}
