   fn main(){
   	let p:f64 = 520000000.0;
   	let n:f64 = 5.0;
   	let r:f64 = 1.0;
   	let a = p * (1.0 + (r / 100.0 )) * n;
   	println!("Amount is {}", a); 
   	let si = a - p;
   	println!("simple interest is {}", si);
}