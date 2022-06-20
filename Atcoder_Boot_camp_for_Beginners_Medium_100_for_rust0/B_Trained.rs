use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [usize; n],
   }
   let mut j = 0;
   let mut ans = -1;
   for i in 1..=n {
      j = a[j] - 1;
      if j == 1 {
         ans = i as i32;
         break;
      }
   }
   println!("{}", ans);
}
