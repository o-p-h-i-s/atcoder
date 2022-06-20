use proconio::input;

fn main() {
   input! { n: usize }
   let mut map = Vec::new();
   for i in 1..=n {
      input! { sp: (String, i32) }
      map.push((sp.0, -sp.1, i));
   }
   map.sort();
   for map in map.into_iter() {
      println!("{}", map.2);
   }
}
