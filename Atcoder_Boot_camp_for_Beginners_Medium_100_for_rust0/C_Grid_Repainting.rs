use proconio::{input, marker::Chars};

fn main() {
   input! {
      h: usize,
      w: usize,
      s: [Chars; h],
   }
   let mut t = vec![vec![false; w]; h];
   for i in 0..h {
      for j in 0..w {
         if 0 < j {
            if s[i][j] == '#' && s[i][j - 1] == '#' {
               t[i][j] = true;
               t[i][j - 1] = true;
            }
         }
      }
   }
   for i in 0..w {
      for j in 0..h {
         if 0 < j {
            if s[j][i] == '#' && s[j - 1][i] == '#' {
               t[j][i] = true;
               t[j - 1][i] = true;
            }
         }
      }
   }
   let mut ans = "Yes";
   for i in 0..h {
      for j in 0..w {
         if (s[i][j] == '#' && !t[i][j]) || (s[i][j] == '.' && t[i][j]) {
            ans = "No";
            break;
         }
      }
   }
   println!("{}", ans);
}
