use proconio::{input, marker::Chars};

fn main() {
   input! {
      abcd: Chars
   }
   let a = abcd[0] as i32 - '0' as i32;
   let b = abcd[1] as i32 - '0' as i32;
   let c = abcd[2] as i32 - '0' as i32;
   let d = abcd[3] as i32 - '0' as i32;
   let mut ans = String::new();
   for i in 0..=1 {
      for j in 0..=1 {
         for k in 0..=1 {
            let n = cul(k, cul(j, cul(i, a, b), c), d);
            if n == 7 {
               let p = &'+'.to_string();
               let m = &'-'.to_string();
               ans += if i == 0 { p } else { m };
               ans += if j == 0 { p } else { m };
               ans += if k == 0 { p } else { m };
               break;
            }
         }
      }
   }
   println!(
      "{}{}{}{}{}{}{}=7",
      a,
      ans.chars().nth(0).unwrap(),
      b,
      ans.chars().nth(1).unwrap(),
      c,
      ans.chars().nth(2).unwrap(),
      d
   );
}

fn cul(flag: i32, a: i32, b: i32) -> i32 {
   if flag == 0 {
      a + b
   } else {
      a - b
   }
}
