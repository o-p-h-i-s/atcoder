use proconio::input;

fn main() {
   input! {
      n: usize,
      h: [i32; n],
   }
   let &max = h.iter().max().unwrap();
   let mut ans = 0;
   for i in 1..=max {
      let mut j = 0;
      while j < n {
         let mut flag = false;
         while j < n && i <= h[j] {
            j += 1;
            flag = true;
         }
         if flag {
            ans += 1;
         }
         j += 1;
      }
   }
   println!("{}", ans);
}

#[allow(dead_code)]
fn gcd(a: i64, b: i64) -> i64 {
   if 0 < a {
      return gcd(b % a, a);
   } else {
      return b;
   }
}

#[allow(dead_code)]
fn lcm(a: i64, b: i64) -> i64 {
   return a / gcd(a, b) * b;
}
