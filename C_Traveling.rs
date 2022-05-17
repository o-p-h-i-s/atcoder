use proconio::input;

fn main() {
   input! {
      n: usize,
      txy: [(i32, i32, i32); n],
   }
   let mut pre = (0, 0, 0);
   let mut ans = "Yes";
   for i in 0..n {
      let l = txy[i].0 - pre.0;
      let r = (txy[i].1 - pre.1).abs() + (txy[i].2 - pre.2).abs();
      let parity = l % 2 == r % 2;
      if !(r <= l && parity) {
         ans = "No";
         break;
      }
      pre = txy[i];
   }
   println!("{}", ans);
}
