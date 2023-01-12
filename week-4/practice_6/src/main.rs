fn main() {
    let n1 = "Electrical".to_string();
    let n2 = " Electonic".to_string();
    let n3 = " inginnering".to_string();
   let n4 = n3.replace("inginnering"," Engineering");
   let n5 = n1 + &n2 + &n4;
   println!("\nThe school of {} is \ninformed by the aspirtion of {} \nprofessional in the areas of desigh,
    and maintainance of \nelectrical control systems.",n5, &n5);
}
