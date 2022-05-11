use proconio::input;

fn main() {
   // input
   input! {
      xy1: (i32, i32),
      xy2: (i32, i32),
   }
   // solve
   let s = xy2.0 - xy1.0;
   let t = xy2.1 - xy1.1;
   let xy3 = (xy2.0 - t, xy2.1 + s);
   let xy4 = (xy3.0 - s, xy3.1 - t);
   // answer
   println!("{} {} {} {}", xy3.0, xy3.1, xy4.0, xy4.1);
}
