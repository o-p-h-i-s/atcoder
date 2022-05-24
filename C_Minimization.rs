use proconio::input;

fn main() {
   input! {
      n: i32,
      k: i32,
      _: [i32; n],
   }
   let ans = (n + k - 3) / (k - 1);
   println!("{}", ans);
}
