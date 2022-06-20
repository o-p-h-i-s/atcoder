use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut ans = 1i64;
   let mut check = (false, false);
   for i in 0..n - 1 {
      if a[i] < a[i + 1] {
         check.0 = true;
      } else if a[i + 1] < a[i] {
         check.1 = true;
      }
      if check.0 && check.1 {
         ans += 1;
         check = (false, false);
      }
   }
   println!("{}", ans);
}
