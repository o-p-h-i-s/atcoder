use proconio::input;

fn main() {
   input! {
      a: i32,
      b: i32,
      c: i32,
      x: i32,
      y: i32,
   }
   let n = x * a + y * b;
   let m = c * 2 * std::cmp::min(x, y) + if y <= x { a * (x - y) } else { b * (y - x) };
   let o = c * 2 * std::cmp::max(x, y);
   println!("{}", std::cmp::min(std::cmp::min(n, m), o));
}
