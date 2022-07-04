fn main() {
    let x =4;
	println!("x is:{}", x);

//scope below
	{
		let x = x -2;
		println!("x is:{}",x);
	}

	println!("x is:{}", x);
}
