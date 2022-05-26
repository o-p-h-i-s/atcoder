use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut b = VecDeque::new();
   if n % 2 == 0 {
      for i in 0..n {
         if i % 2 == 0 {
            b.push_back(a[i]);
         } else {
            b.push_front(a[i]);
         }
      }
   } else {
      for i in 0..n {
         if i % 2 == 1 {
            b.push_back(a[i]);
         } else {
            b.push_front(a[i]);
         }
      }
   }
   println!("{}", b.iter().join(" "));
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
