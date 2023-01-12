fn main() {
    let name =  "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let addr:&str = "km 52 Lekki Epe Expressway, Ibeju, lekki, Lagos";
    println!("Name{}", name);
    println!("University {}, \nAddress: {}", uni, addr);
    let department:&'static str = "computeer Science";
    let school:&'static str = "school of Science and Technology";
    println!("Department: {}, \n School: {}", department, school);
}
