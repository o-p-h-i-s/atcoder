use proconio::input;

fn main() {
   input! {
      n: i64,
      a: [i64; n],
   }
   let mut two = 0;
   let mut four = 0;
   let mut odd = 0;
   for &a in a.iter() {
      if a % 4 == 0 {
         four += 1;
      } else if a % 2 == 0 {
         two += 1;
      } else {
         odd += 1;
      }
   }
   let mut ans = "Yes";
   if two == 0 {
      if four + 1 < odd {
         ans = "No";
      }
   } else {
      if four < odd {
         ans = "No";
      }
   }
   println!("{}", ans);
}
