fn main(){
	let mut base_string = String::from("World!");
	println!("Original String: {:?}", base_string);
	
	// append the prefix to the base string
	base_string.insert_str(0, "Hello ");
	println!("Single Append String: {:?}", base_string);
}