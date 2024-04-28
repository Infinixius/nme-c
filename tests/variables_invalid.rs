
use nme_c::{
	compiler::compile,
	parser::{parse, Node, Type},
	tokenizer::{tokenize, Token},
};

#[test]
#[should_panic]
fn void_int() {
	const SOURCE: &str = "void x = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn void_char() {
	const SOURCE: &str = "void x = 'a';";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn int_type_missing() {
	const SOURCE: &str = "x = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn char_type_missing() {
	const SOURCE: &str = "x = 'a';";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn char_type_mismatch() {
	const SOURCE: &str = "char x = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn int_type_mismatch() {
	const SOURCE: &str = "int x = 'a';";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn underscore_in_variable_name() {
	const SOURCE: &str = "int integer_a = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}

#[test]
#[should_panic]
fn number_in_variable_name() {
	const SOURCE: &str = "int x1 = 1;";

	let tokens = tokenize(SOURCE);

	let tree = parse(tokens, nme_c::parser::Context::Program);
}