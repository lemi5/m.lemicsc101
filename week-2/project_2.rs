fn main(){
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;
	let x = t + m + h + d + a;
	println!("The sum of the sale is {}", x);
	let z:f64 = x / 5.0;
	println!("The sales average is {}", z);
}
