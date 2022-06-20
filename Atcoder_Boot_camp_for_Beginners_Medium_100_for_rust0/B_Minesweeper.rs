use proconio::input;

fn main() {
   input! {
      h: usize,
      w: usize,
   }
   let mut s = vec![vec!['.'; w + 2]; h + 2];
   for i in 1..=h {
      input! { t: String }
      let n = '.'.to_string() + &t + &'.'.to_string();
      s[i] = n.chars().collect::<Vec<char>>();
   }
   let mut ans = vec![vec![0; w]; h];
   for i in 1..=h {
      for j in 1..=w {
         if s[i][j] != '#' {
            let mut cnt = 0;
            for k in 0..=2 {
               for l in 0..=2 {
                  if s[i + k - 1][j + l - 1] == '#' {
                     cnt += 1;
                  }
               }
            }
            ans[i - 1][j - 1] += cnt;
         }
      }
   }
   for i in 0..h {
      let mut n = String::new();
      for j in 0..w {
         if s[i + 1][j + 1] == '#' {
            n += &'#'.to_string();
         } else {
            n += &ans[i][j].to_string();
         }
      }
      println!("{}", n);
   }
}
