use proconio::input;

fn main() {
   input! {
      n: usize,
      m: usize,
      ab: [(i32, i32); n],
      cd: [(i32, i32); m],
   }
   for i in 0..n {
      let mut ans = (std::i32::MAX, 0);
      for j in 0..m {
         let tmp = (ab[i].0 - cd[j].0).abs() + (ab[i].1 - cd[j].1).abs();
         if tmp < ans.0 {
            ans = (tmp, j + 1);
         }
      }
      println!("{}", ans.1);
   }
}
