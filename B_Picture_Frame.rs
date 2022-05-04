use proconio::input;

fn main() {
   // input
   input! {
      h: i32,
      w: i32,
      a: [String; h],
   }
   // solve
   // answer
   println!("{}", (0..w + 2).map(|_| '#').collect::<String>());
   for vec in a.into_iter() {
      println!("#{}#", vec);
   }
   println!("{}", (0..w + 2).map(|_| '#').collect::<String>());
}
