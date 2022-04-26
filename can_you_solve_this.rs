use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      m: usize,
      c: i32,
      b: [i32; m],
      a: [[i32; m]; n],
   }
   // solve
   let mut ans = 0;
   for i in 0..n {
      let mut tmp = c;
      for j in 0..m {
         tmp += a[i][j] * b[j];
      }
      if 0 < tmp {
         ans += 1;
      }
   }
   // answer
   println!("{}", ans);
}
