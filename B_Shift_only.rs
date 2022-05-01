use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      a: [i64; n],
   }
   // solve
   let mut ans = std::i64::MAX;
   for val in a.into_iter() {
      ans = std::cmp::min(ans, is_ok(val));
   }
   // answer
   println!("{}", ans);
}

fn is_ok(mut x: i64) -> i64 {
   let mut res = 0;
   while x % 2 == 0 {
      res += 1;
      x /= 2;
   }
   res
}
