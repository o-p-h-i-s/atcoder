use proconio::input;

fn main() {
   input! {
      n: usize,
      a: [i32; n],
   }
   let mut ans = 0;
   for a in a.into_iter() {
      ans += cnt(a);
   }
   println!("{}", ans);
}

fn cnt(mut n: i32) -> i32 {
   let mut res = 0;
   while n % 2 == 0 {
      res += 1;
      n /= 2;
   }
   res
}
