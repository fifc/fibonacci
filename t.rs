
use std::time::{SystemTime};

fn fibonacci(i: i32) -> i32
{
	if i < 2 {
		return i;
	}
	return fibonacci(i - 2) + fibonacci(i - 1);
}
 
fn main()
{
	let start = SystemTime::now();
	let fibn = fibonacci(45);
	println!("{:?}", fibn);
	match start.elapsed() {
		Ok(elapsed) => {
			println!("elapsed: {}s{}ms", elapsed.as_secs(), elapsed.subsec_nanos()/1000000);
		}
		Err(e) => {
			println!("{:?}", e);
		}
	}
}
