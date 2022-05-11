use proconio::input;

fn main() {
   input! {
      mut n: i64,
   }
   n += 1;
   let mut digits = 0;
   while 10 < n {
      n /= 10;
      digits += 1;
   }
   let ans = n - 1 + digits * 9;
   println!("{}", ans);
}
