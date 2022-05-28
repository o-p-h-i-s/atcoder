#[allow(unused_imports)]
use proconio::{input, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
   input! {
      _: usize,
      a: usize,
      b: usize,
      c: usize,
      d: usize,
      s: Chars,
   }
   let mut ans = "Yes";
   if !(d < c) {
      for &(i, j) in [(b - 1, d - 1), (a - 1, c - 1)].iter() {
         for k in i..j - 1 {
            if s[k + 1] == '#' && s[k] == '#' {
               ans = "No";
               break;
            }
         }
      }
   } else {
      let mut flag = true;
      for i in a - 1..c - 2 {
         if b - 1 <= i && i <= d - 1 {
            if s[i - 1] == '.' && s[i] == '.' && s[i + 1] == '.' {
               flag = false;
            }
         }
         if s[i + 1] == '#' && s[i] == '#' {
            ans = "No";
            break;
         }
      }
      if flag {
         ans = "No";
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
