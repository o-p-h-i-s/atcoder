use itertools::Itertools;
use proconio::input;

fn main() {
   // input
   input! {
      menu: [i32; 5],
   }
   // solve
   let mut ans = std::i32::MAX;
   for perm in menu.into_iter().permutations(5) {
      let mut tmp = 0;
      for (i, val) in perm.into_iter().enumerate() {
         tmp += val;
         if i == 4 {
            break;
         }
         tmp = (tmp + 9) / 10 * 10;
      }
      ans = std::cmp::min(ans, tmp);
   }
   // answer
   println!("{}", ans);
}
