fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	let a = p * (1.00 - (r / 100.00) * n);
	let i = a - p;
	println!("The compound interest is {}", i);
}