use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      mut x: i32,
      mut a: [i32; n],
   }
   // solve
   a.sort();
   let mut ans = 0;
   for i in 0..n {
      x -= a[i];
      if 0 <= x {
         ans += 1;
      } else {
         break;
      }
   }
   if 0 < x {
      ans -= 1;
   }
   // answer
   println!("{}", ans);
}
