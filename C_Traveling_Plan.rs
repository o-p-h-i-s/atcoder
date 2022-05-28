use proconio::input;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
   input! {
      n: usize,
      mut a: [i32; n],
   }
   a.insert(n, 0);
   a.insert(0, 0);
   let mut sum = 0;
   for i in 0..n + 1 {
      sum += (a[i + 1] - a[i]).abs();
   }
   for i in 1..=n {
      let pre = (a[i + 1] - a[i]).abs() + (a[i] - a[i - 1]).abs();
      let post = (a[i + 1] - a[i - 1]).abs();
      println!("{}", sum - pre + post);
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
