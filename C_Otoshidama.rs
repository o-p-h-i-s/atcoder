use proconio::input;

fn main() {
   input! {
      n: i32,
      y: i32,
   }
   let mut ans = (-1, -1, -1);
   for a in 0..=n {
      for b in 0..=n {
         let c = n - (a + b);
         let tmp = 10000 * a + 5000 * b + 1000 * c;
         if tmp == y && a + b + c == n && 0 <= c {
            ans = (a, b, c);
            break;
         }
      }
   }
   println!("{} {} {}", ans.0, ans.1, ans.2);
}
