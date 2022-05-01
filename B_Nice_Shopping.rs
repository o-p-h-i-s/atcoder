use proconio::input;

fn main() {
   // input
   input! {
      an: usize,
      bn: usize,
      mn: usize,
      a: [usize; an],
      b: [usize; bn],
      xyc: [(usize, usize, usize); mn],
   }
   // solve
   let mut ans = a.iter().min().unwrap() + b.iter().min().unwrap();
   for (x, y, c) in xyc.into_iter() {
      ans = std::cmp::min(ans, a[x - 1] + b[y - 1] - c);
   }
   // answer
   println!("{}", ans);
}
