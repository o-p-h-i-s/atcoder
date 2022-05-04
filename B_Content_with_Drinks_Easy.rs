use proconio::input;

fn main() {
   input! {
      n: usize,
      t: [i32; n],
      m: usize,
      px: [(usize, i32); m],
   }
   let def = t.iter().sum::<i32>();
   for (p, x) in px.into_iter() {
      let ans = def - t[p - 1] + x;
      println!("{}", ans);
   }
}
