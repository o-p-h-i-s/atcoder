use proconio::input;

fn main() {
   input! {
      n: usize,
      s: i64,
      t: [i64; n],
   }
   let mut ans = s;
   for i in 1..n {
      ans += std::cmp::min(s, t[i] - t[i - 1]);
   }
   println!("{}", ans);
}
