use proconio::input;

fn main() {
   input! {
      s: String,
   }
   let s = s.chars().collect::<Vec<char>>();
   let b = String::from("keyence");
   let mut ans = "NO";
   for i in 0..s.len() {
      for j in 0..(s.len() - i) {
         let t = s[..i].iter().collect::<String>();
         let u = s[i + j..].iter().collect::<String>();
         let a = t + &u;
         if a == b {
            ans = "YES";
            break;
         }
      }
   }
   println!("{}", ans);
}
