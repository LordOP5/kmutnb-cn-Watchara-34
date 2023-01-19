fn main() {
    
    let string = "this cat this bat this rat";
    let mut split : Vec<&str> = string.split(" ").collect();
    let uniqed = uniqe(split);
    println!("{:?}", uniqed);
    let mut length = uniqed.len();
    println!("{}", length);
}

fn uniqe(split: Vec<&str>) -> Vec<&str> {
    let mut uniqe = Vec::new();
    for i in &split {
        if !uniqe.contains(i) {
            uniqe.push(i);
        }
    }
    uniqe
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_word() {
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       assert_eq!(split,["this","cat","this","bat","this","rat"])
   }
   #[test]
   fn test_unique() {
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       let uniqed = uniqe(split);
       assert_eq!(uniqed,["this","cat","bat","rat"])
   }
   #[test]
   fn test_count() {
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       let uniqed = uniqe(split);
       let mut length = uniqed.len();
       assert_eq!(length,4)
   }
}