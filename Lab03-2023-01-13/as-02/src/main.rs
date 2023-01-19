fn main() {

    let number =100;
    let mut sum=(number/2)*(number+1);
    println!("The sum of the first {} natural numbers is: {}", number, sum);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_sum() {
       let number =100;
       let mut sum=(number/2)*(number+1);
       assert_eq!(sum,5050)
   }
}