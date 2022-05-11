use proconio::input;

fn main() {
   input! {
      n: usize,
      h: [i32; n],
   }
   let mut prev = h[0];
   let mut ans = "Yes";
   for h in h.into_iter() {
      if prev <= h - 1 {
         prev = h - 1;
      } else if prev <= h {
         prev = h;
      } else {
         ans = "No";
         break;
      }
   }
   println!("{}", ans);
}
