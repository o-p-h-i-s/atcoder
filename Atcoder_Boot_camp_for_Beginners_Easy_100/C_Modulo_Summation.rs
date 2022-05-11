use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i64; n],
   }
   let mut ans = 0;
   for &a in a.iter() {
      ans += a - 1;
   }
   println!("{}", ans);
}
