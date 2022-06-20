use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      n: usize,
      d: [i32; n],
      m: usize,
      t: [i32; m],
   }
   let mut e = HashMap::new();
   let mut u = HashMap::new();
   for &d in d.iter() {
      *e.entry(d).or_insert(0) += 1;
   }
   for &t in t.iter() {
      *u.entry(t).or_insert(0) += 1;
   }
   let mut ans = "YES";
   for (t, val) in u.iter() {
      match e.get(t) {
         Some(v) => {
            if v < val {
               ans = "NO";
               break;
            }
         }
         _ => {
            ans = "NO";
            break;
         }
      }
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
