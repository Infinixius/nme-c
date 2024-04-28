use std::collections::HashSet;

use nme_c::{
	compiler::compile,
	parser::{parse, Node, Type},
	tokenizer::{tokenize, Token},
};

#[test]
#[should_panic]
fn function_no_body() {
	const SOURCE: &str = "void main();";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn function_no_return_type() {
	const SOURCE: &str = "main() {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn function_no_name() {
	const SOURCE: &str = "void() {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn function_no_args() {
	const SOURCE: &str = "void main {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn number_in_function_name() {
	const SOURCE: &str = "void main1() {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn underscore_in_function_name() {
	const SOURCE: &str = "void main_func() {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn number_in_function_arg_name() {
	const SOURCE: &str = "void main(int a1) {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn underscore_in_function_arg_name() {
	const SOURCE: &str = "void main(int a_b) {}";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}