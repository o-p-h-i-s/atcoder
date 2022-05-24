use proconio::input;

fn main() {
   input! {
      n: usize,
      x: [i64; n + 1],
   }
   let mut d = (x[1] - x[0]).abs();
   for i in 0..n {
      d = gcd(d, (x[i + 1] - x[i]).abs());
   }
   println!("{}", d);
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
