// This compiler does not support floats at all

use nme_c::{
	compiler::compile,
	parser::{parse, Node, Type},
	tokenizer::{tokenize, Token},
};

#[test]
#[should_panic]
fn float_keyword() {
	const SOURCE: &str = "float f = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn decimal_point() {
	const SOURCE: &str = "int a = 1.0;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn float_function() {
	const SOURCE: &str = "float f() { return 1.0; }";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn decimal_point_in_arithmetic() {
	const SOURCE: &str = "int a = 1 + 1.0;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn decimal_point_in_function_return() {
	const SOURCE: &str = "int f() { return 1.0; }";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn decimal_point_in_function_call() {
	const SOURCE: &str = "f(1.0);";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}