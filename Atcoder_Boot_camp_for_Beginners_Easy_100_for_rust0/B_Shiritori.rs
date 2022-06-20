use proconio::input;

fn main() {
   input! {
      n: usize,
      mut w: [String; n],
   }
   let mut cnt = Vec::new();
   cnt.push(w.remove(0));
   let mut ans = "Yes";
   for w in w.into_iter() {
      if cnt.last().unwrap().chars().last().unwrap() == w.chars().next().unwrap()
         && !cnt.contains(&w)
      {
         cnt.push(w);
      } else {
         ans = "No";
         break;
      }
   }
   println!("{}", ans);
}
