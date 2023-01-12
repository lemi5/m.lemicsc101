fn main() {
    let fullname = "Pan Atlantic UNiversity     ";
    println!();
    println!("Name: {}", fullname);
    println!("Before trim ");
    println!("length is {}",fullname.len());
    println!();
   println!("After trim ");
   println!("Length is{}", fullname.trim().len());
}
