use proconio::input;

fn main() {
   // input
   input! {
      n: usize,
   }
   let mut a = vec![0; n];
   for i in 0..n {
      input! { student: usize }
      a[student - 1] = i + 1;
   }
   // answer
   for val in a.iter() {
      print!("{} ", val);
   }
}
