use proconio::input;

fn main() {
   // input
   input! {
      mut a: i32,
      mut b: i32,
      mut c: i32,
   }
   // solve
   let mut ans = 0;
   let mut flag = true;
   while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 && flag {
      ans += 1;
      if a == b && b == c {
         flag = false;
         ans = -1;
      }
      let ah = a / 2;
      let bh = b / 2;
      let ch = c / 2;
      a = bh + ch;
      b = ch + ah;
      c = ah + bh;
   }
   // answer
   println!("{}", ans);
}
