use proconio::input;

fn main() {
   // input
   input! {
      n: i64,
   }
   // solve
   let mut ans: (i64, i64, i64) = (2, 1, 3);
   for _ in 0..n {
      ans = (ans.1, ans.2, ans.1 + ans.2);
   }
   // answer
   println!("{}", ans.0);
}
