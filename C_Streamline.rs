use proconio::input;

fn main() {
   input! {
      n: usize,
      m: usize,
      mut x: [i32; m],
   }
   x.sort();
   let ans = if m <= n {
      0
   } else {
      let mut tmp = x[m - 1] - x[0];
      let mut o = vec![0; m - 1];
      for i in 0..m - 1 {
         o[i] = x[i + 1] - x[i];
      }
      o.sort();
      o.reverse();
      for i in (0..n - 1).rev() {
         tmp -= o[i];
      }
      tmp
   };
   println!("{}", ans);
}

#[allow(dead_code)]
fn gcd(a: i64, b: i64) -> i64 {
   if 0 < a {
      return gcd(b % a, a);
   } else {
      return b;
   }
}

#[allow(dead_code)]
fn lcm(a: i64, b: i64) -> i64 {
   return a / gcd(a, b) * b;
}
