use proconio::input;

fn main() {
   input! {
      c: [[i32; 3]; 3],
   }
   let mut a = vec![0; 3];
   for i in 0..3 {
      a[i] = *c[i].iter().min().unwrap();
   }
   if (c[0][0] - a[0]) == (c[1][0] - a[1])
      && (c[1][0] - a[1]) == (c[2][0] - a[2])
      && (c[0][1] - a[0]) == (c[1][1] - a[1])
      && (c[1][1] - a[1]) == (c[2][1] - a[2])
      && (c[0][2] - a[0]) == (c[1][2] - a[1])
      && (c[1][2] - a[1]) == (c[2][2] - a[2])
   {
      println!("Yes");
   } else {
      println!("No");
   }
}
