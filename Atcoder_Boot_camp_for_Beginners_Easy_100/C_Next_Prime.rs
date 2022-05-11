use proconio::input;

fn main() {
   // input
   input! {
      x: i32,
   }
   // solve
   let mut ans = 0;
   for i in x..900000 {
      if is_ok(&i) {
         ans = i;
         break;
      }
   }
   // answer
   println!("{}", ans);
}

fn is_ok(&x: &i32) -> bool {
   let mut res = true;
   for i in 2..x {
      if x % i == 0 {
         res = false;
      }
   }
   res
}
