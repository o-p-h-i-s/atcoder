use proconio::{input, marker::Chars};

fn main() {
   input! {
      s: Chars
   }
   let n = s.len();
   let mut a = vec![0; n + 1];
   for i in 0..n {
      if s[i] == '<' {
         a[i + 1] = a[i] + 1;
      }
   }
   for i in (0..n).rev() {
      if s[i] == '>' {
         a[i] = std::cmp::max(a[i], a[i + 1] + 1);
      }
   }
   let ans = a.iter().sum::<i64>();
   println!("{}", ans);
}
