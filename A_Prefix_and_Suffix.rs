use proconio::input;

fn main() {
   input! {
      n: usize,
      s: String,
      t: String,
   }
   let mut m = 0;
   for i in (1..=n).rev() {
      let u = &s[n - i..];
      let v = &t[..i];
      if u == v {
         m = i;
      }
   }
   let ans = if s != t { (s + &t[m..]).len() } else { n };
   println!("{}", ans);
}
