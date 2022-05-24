use proconio::input;

fn main() {
   input! {
      k: i64,
      a: i64,
      b: i64,
   }
   let mut ans = k + 1;
   if 2 < b - a && a - 1 < k {
      ans = a + (b - a) * ((k - (a - 1)) / 2) + ((k - (a - 1)) % 2);
   }
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
