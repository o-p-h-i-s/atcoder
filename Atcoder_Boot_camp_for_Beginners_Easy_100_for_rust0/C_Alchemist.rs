use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      mut v: [f64; n],
   }
   // solve
   v.sort_by(|a, b| a.partial_cmp(b).unwrap());
   let mut ans = v[0];
   for val in v.iter() {
      ans = (ans + val) / 2.0;
   }
   // answer
   println!("{}", ans);
}
