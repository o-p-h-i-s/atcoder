use proconio::input;

fn main() {
   input! {
      n: usize,
      m: usize,
      mut sc: [(usize, i32); m],
   }
   let a = if n == 1 { 0 } else { 10i32.pow(n as u32 - 1) };
   let b = 10i32.pow(n as u32);
   sc.sort();
   let mut ans = -1;
   for i in a..b {
      let mut v = Vec::new();
      for j in (0..n).rev() {
         let x = (i / 10i32.pow(j as u32)) % 10;
         v.push(x);
      }
      let mut flag = true;
      for &(s, c) in sc.iter() {
         if v[s - 1] != c {
            flag = false;
            break;
         }
      }
      if flag {
         ans = i;
         break;
      }
   }
   println!("{}", ans);
}
