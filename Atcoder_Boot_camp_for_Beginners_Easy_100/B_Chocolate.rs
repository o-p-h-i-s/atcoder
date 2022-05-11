use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      d: i32,
      x: i32,
      a: [i32; n],
   }
   // solve
   let mut ans = x;
   for val in a.into_iter() {
      let mut day = 1;
      while day <= d {
         ans += 1;
         day += val;
      }
   }
   // answer
   println!("{}", ans);
}
