use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      h: [i32; n],
   }
   // solve
   let mut ans = 0;
   let mut cnt = (h[0], -1);
   for h in h.into_iter() {
      if h <= cnt.0 {
         cnt.1 += 1;
         cnt.0 = h;
      } else {
         cnt = (h, 0);
         ans = std::cmp::max(ans, cnt.1);
      }
      ans = std::cmp::max(ans, cnt.1);
   }
   // answer
   println!("{}", ans);
}
