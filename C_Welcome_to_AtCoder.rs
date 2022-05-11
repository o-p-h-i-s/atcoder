use proconio::input;

fn main() {
   input! {
      n: usize,
      m: usize,
      ps: [(usize, String); m],
   }
   let mut submits = vec![0; n];
   let mut ans = (0, 0);
   for (p, s) in ps.into_iter() {
      if submits[p - 1] == -1 {
         continue;
      }
      if s == "WA" {
         submits[p - 1] += 1;
      } else {
         ans.0 += 1;
         ans.1 += submits[p - 1];
         submits[p - 1] = -1;
      }
   }
   println!("{} {}", ans.0, ans.1);
}
