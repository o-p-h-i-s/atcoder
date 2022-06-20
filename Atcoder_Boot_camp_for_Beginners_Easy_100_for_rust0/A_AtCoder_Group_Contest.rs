use proconio::input;

fn main() {
   input! {
      n: usize,
      mut a: [i32; 3 * n],
   }
   a.sort();
   a.reverse();
   let mut ans: i64 = 0;
   let mut j = 0;
   for (i, a) in a.into_iter().enumerate() {
      if i % 2 == 1 && j < n {
         ans += a as i64;
         j += 1;
      }
   }
   println!("{}", ans);
}
