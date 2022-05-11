use itertools::Itertools;
use proconio::input;

fn main() {
   input! {
      qhsd: [i64; 4],
      n: i64,
   }
   let mut tea = Vec::new();
   for (i, val) in qhsd.into_iter().enumerate() {
      tea.push((val, (2i64.pow(i as u32))));
   }
   let mut ans = std::i64::MAX;
   for vec in tea.into_iter().permutations(4) {
      let mut tmp = 0;
      let mut m = 4 * n;
      for (y, l) in vec.into_iter() {
         tmp += (m / l) * y;
         m = m % l;
      }
      ans = std::cmp::min(ans, tmp);
   }
   println!("{}", ans);
}
