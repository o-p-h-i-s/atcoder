use proconio::input;
use std::collections::HashMap;

fn main() {
   input! {
      s: String,
   }
   let s = s.chars().collect::<Vec<char>>();
   let mut t = HashMap::new();
   t.entry('a').or_insert(0);
   t.entry('b').or_insert(0);
   t.entry('c').or_insert(0);
   for &c in s.iter() {
      *t.entry(c).or_insert(0) += 1;
   }
   let mut t = t.iter().collect::<Vec<(&char, &i32)>>();
   t.sort_by(|a, &b| a.1.cmp(b.1));
   let flag = ((t[0].1 - t[1].1).abs() <= 1)
      && ((t[1].1 - t[2].1).abs() <= 1)
      && ((t[2].1 - t[0].1).abs() <= 1);
   let ans;
   if flag {
      ans = "YES";
   } else {
      ans = "NO";
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
