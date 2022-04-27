use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      mut d: [i32; n],
   }
   // solve
   d.sort();
   let ans = d[n / 2] - d[n / 2 - 1];
   println!("{}", ans);
}
