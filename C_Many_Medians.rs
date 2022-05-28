use proconio::input;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
   input! {
      n: usize,
      x: [i64; n],
   }
   let mut y = x.clone();
   y.sort();
   for &x in x.iter() {
      if x <= y[n / 2 - 1] {
         println!("{}", y[n / 2]);
      } else {
         println!("{}", y[n / 2 - 1]);
      }
   }
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

trait RunLengthEncoding {
   fn rle(&self) -> Vec<(char, i64)>;
}

impl RunLengthEncoding for String {
   fn rle(&self) -> Vec<(char, i64)> {
      let s = self.chars().collect::<Vec<char>>();
      let mut res = Vec::new();
      let mut pre = ' ';
      let mut count = 0;
      for c in s.into_iter() {
         if c == pre {
            count += 1;
         } else {
            if pre != ' ' {
               res.push((pre, count));
            }
            pre = c;
            count = 1;
         }
      }
      res.push((pre, count));
      res
   }
}

impl RunLengthEncoding for Vec<char> {
   fn rle(&self) -> Vec<(char, i64)> {
      let mut res = Vec::new();
      let mut pre = ' ';
      let mut count = 0;
      for &c in self.into_iter() {
         if c == pre {
            count += 1;
         } else {
            if pre != ' ' {
               res.push((pre, count));
            }
            pre = c;
            count = 1;
         }
      }
      res.push((pre, count));
      res
   }
}
