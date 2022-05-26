use proconio::input;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
   input! {
      n: usize,
      a: [i64; n],
   }
   let mut s = HashMap::new();
   for &a in a.iter() {
      *s.entry(a).or_insert(0) += 1;
   }
   let mut t = HashMap::new();
   for (&key, &val) in s.iter() {
      if 2 <= val {
         *t.entry(key).or_insert(0) += val;
      }
   }
   let mut t = t.into_iter().collect::<Vec<(i64, i32)>>();
   t.sort();
   t.reverse();
   let ans = if 1 <= t.len() && 4 <= t[0].1 {
      t[0].0 * t[0].0
   } else if 2 <= t.len() {
      t[0].0 * t[1].0
   } else {
      0
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
