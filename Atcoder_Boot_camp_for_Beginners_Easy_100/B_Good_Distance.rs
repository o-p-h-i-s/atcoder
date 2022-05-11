use proconio::input;

fn main() {
   input! {
      n: usize,
      d: usize,
      x: [[i32; d]; n],
   }
   let mut ans = 0;
   for i in 0..n {
      for j in i + 1..n {
         let a = x[i]
            .iter()
            .zip(x[j].iter())
            .map(|(&a, &b)| (a - b).abs() * (a - b).abs())
            .sum::<i32>();
         if is_ok(a) {
            ans += 1;
         }
      }
   }
   println!("{}", ans);
}

fn is_ok(a: i32) -> bool {
   let mut res = false;
   let mut n = 1;
   while n * n <= a {
      if n * n == a {
         res = true;
      }
      n += 1;
   }
   res
}
