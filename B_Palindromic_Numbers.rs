use proconio::input;

fn main() {
   // input
   input! {
      a: i32,
      b: i32,
   }
   // solve
   let mut ans = 0;
   for i in a..=b {
      let j: i32 = i
         .to_string()
         .chars()
         .rev()
         .collect::<String>()
         .parse()
         .unwrap();
      if i == j {
         ans += 1;
      }
   }
   // answer
   println!("{}", ans);
}
