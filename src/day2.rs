
pub fn main() {
    println!("{}", "this is main")

}


#[cfg(test)]
mod tests {
    use super::*;
   #[test]
   fn test_dummy(){
       println!("{}", "this is test dummy")
   }
    
}
