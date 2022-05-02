use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      k: i32,
      q: usize,
      a: [usize; q],
   }
   // solve
   let mut entry = vec![k - q as i32; n];
   for val in a.into_iter() {
      entry[val - 1] += 1;
   }
   // answer
   for ans in entry.into_iter() {
      let ans = if ans <= 0 { "No" } else { "Yes" };
      println!("{}", ans);
   }
}
