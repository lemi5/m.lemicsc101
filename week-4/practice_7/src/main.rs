fn main() {
   

let a:i32 = 10;
let b:i32 = 20;
println!("value of A:{}",a);
println!("value of B:{}",b);

let mut res = a>b;
println!("A greater than B: {}",res);
res = a<b;
println!("A lesser than B: {}",res);
 
 res = a>=b;
 println!("A greater or equal to B: {}",res);

 res = a==b;
 println!("A is equal to B: {}",res);

 res = a!=b;
 println!("A is not equal to B: {}",res);
}
