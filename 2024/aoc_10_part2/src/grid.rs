struct Grid<T> {
	data: Vec<T>;
	width: usize;
	height: usize;
}

// `<T>` Must precede the type to remain generic
impl<T> Grid<T> {
	fn new(width: usize, height: usize) -> Self {
		let object = Self { data: Vec::new(), width: width, height: height };

		for _ in 0..(width * height) {
			object.push(<T>::new());
		}

		object
	}
}

