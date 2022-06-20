use proconio::input;

fn main() {
   input! {
      n: i64,
      a: i64,
      b: i64,
   }
   let ans = if (a - b) % 2 == 0 {
      (b - (a + b) / 2).abs()
   } else {
      let x = a + (b - a - 1) / 2;
      let y = n - b + 1 + (b - a - 1) / 2;
      std::cmp::min(x, y)
   };
   println!("{}", ans);
}
