use proconio::input;

fn main() {
   // input
   input! {
      mut h: i64,
   }
   // solve
   let ans = attack(h, 0);
   // answer
   println!("{}", ans);
}

fn attack(mut h: i64, mut res: i64) -> i64 {
   if h == 0 {
      return res;
   }
   h /= 2;
   res = 1 + attack(h, res) * 2;
   res
}
