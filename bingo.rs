use proconio::input;

fn main() {
   // input
   input! {
      a: [[i32; 3]; 3],
      n: usize,
      b: [i32; n],
   }
   // solve
   let mut bingo = [[false; 3]; 3];
   for i in 0..3 {
      for j in 0..3 {
         for k in 0..n {
            if a[i][j] == b[k] {
               bingo[i][j] = true;
            }
         }
      }
   }
   let mut ans = false;
   for i in 0..3 {
      if bingo[i][0] && bingo[i][1] && bingo[i][2] {
         ans = true;
      }
      if bingo[0][i] && bingo[1][i] && bingo[2][i] {
         ans = true;
      }
   }
   if bingo[0][0] && bingo[1][1] && bingo[2][2] {
      ans = true;
   }
   if bingo[2][0] && bingo[1][1] && bingo[0][2] {
      ans = true;
   }
   // answer
   println!("{}", if ans { "Yes" } else { "No" });
}
