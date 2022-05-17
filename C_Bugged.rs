use proconio::input;

fn main() {
   input! {
      n: usize,
      mut s: [i32; n],
   }
   s.sort();
   let mut sum = s.iter().sum::<i32>();
   let mut ans = sum;
   let mut flag = false;
   for &s in s.iter() {
      if sum % 10 != 0 {
         break;
      }
      if s % 10 == 0 {
         flag = true;
         continue;
      }
      if sum - s % 10 != 0 {
         sum -= s;
         break;
      }
   }
   if flag && ans == sum {
      ans = 0;
   } else {
      ans = sum;
   }
   println!("{}", ans);
}
