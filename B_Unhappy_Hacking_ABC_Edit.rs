use proconio::input;

fn main() {
   input! {
      s: String
   }
   let s = s.chars().collect::<Vec<char>>();
   let mut ans = Vec::new();
   for &c in s.iter() {
      if c != 'B' {
         ans.push(c);
      } else if ans.len() != 0 {
         ans.pop();
      }
   }
   let ans = ans.iter().collect::<String>();
   println!("{}", ans);
}
