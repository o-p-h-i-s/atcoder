use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
      m: usize,
      x: usize,
      a: [usize; m],
   }
   // solve
   let mut ans = 0;
   let mut map = vec![0; n + 1];
   for cost in a.into_iter() {
      map[cost] = if cost == 0 || cost == x || cost == n {
         0
      } else {
         1
      };
   }
   for i in x..n + 1 {
      ans += map[i];
   }
   let mut tmp = 0;
   for i in (0..x).rev() {
      tmp += map[i];
   }
   ans = std::cmp::min(ans, tmp);

   // answer
   println!("{}", ans);
}
