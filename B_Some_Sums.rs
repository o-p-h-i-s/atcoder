use proconio::input;

fn main() {
   input! {
      n: i32,
      a: i32,
      b: i32,
   }
   let mut ans = 0;
   for i in 1..=n {
      let x = i.to_string();
      let x = x
         .chars()
         .map(|c| c.to_digit(10).unwrap() as i32)
         .sum::<i32>();
      if a <= x && x <= b {
         ans += i;
      }
   }
   println!("{}", ans);
}
