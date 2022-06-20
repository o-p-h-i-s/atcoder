use proconio::input;

fn main() {
   input! {
      mut abc: [i64; 3],
   }
   abc.sort();
   let ans = abc[0] * abc[1] * ((abc[2] + 1) / 2 - abc[2] / 2);
   println!("{}", ans);
}
