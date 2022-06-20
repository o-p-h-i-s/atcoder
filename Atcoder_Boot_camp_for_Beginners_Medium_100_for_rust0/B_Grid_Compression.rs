use proconio::{input, marker::Chars};

fn main() {
   input! {
      h: usize,
      w: usize,
      a: [Chars; h],
   }
   let mut si = vec![false; h];
   let mut sj = vec![false; w];
   for i in 0..h {
      let mut f = false;
      for j in 0..w {
         if a[i][j] == '#' {
            f = true;
            break;
         }
      }
      if f {
         si[i] = true;
      }
   }
   for i in 0..w {
      let mut f = false;
      for j in 0..h {
         if a[j][i] == '#' {
            f = true;
            break;
         }
      }
      if f {
         sj[i] = true;
      }
   }
   for i in 0..h {
      for j in 0..w {
         if si[i] && sj[j] {
            print!("{}", a[i][j]);
         }
      }
      if si[i] {
         println!("");
      }
   }
}
