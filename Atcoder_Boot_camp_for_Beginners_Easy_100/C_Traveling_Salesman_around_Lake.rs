use proconio::input;

fn main() {
   // input
   input! {
      k: i32,
      n: usize,
      mut a: [i32; n],
   }
   // solve
   let mut a_clone = a.clone();
   for i in 0..n {
      a_clone[i] += k;
   }
   a.append(&mut a_clone);
   let mut ans = 0;
   for i in 0..(a.len() - 1) {
      ans = std::cmp::max(ans, a[i + 1] - a[i]);
   }
   // answer
   ans = k - ans;
   println!("{}", ans);
}
