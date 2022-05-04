use proconio::input;

fn main() {
   input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m],
   }
   let mut map = vec![Vec::new(); n + 1];
   for (a, b) in ab.into_iter() {
      map[a].push(b);
      map[b].push(a);
   }
   for i in 1..=n {
      println!("{}", map[i].len());
   }
}
