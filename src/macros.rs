#[macro_export]
macro_rules! debug {
	($($arg:tt)*) => {
		use colored::Colorize;
		print!("{} {}", "DEBUG:".green(), format!($($arg)*))
	}
}

#[macro_export]
macro_rules! debugln {
	($($arg:tt)*) => {
		use colored::Colorize;
		println!("{} {}", "DEBUG:".green(), format!($($arg)*))
	}
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => {
		use colored::Colorize;
		eprint!("{} {}", "ERROR:".red(), format!($($arg)*))
	}
}

#[macro_export]
macro_rules! errorln {
	($($arg:tt)*) => {
		use colored::Colorize;
		eprintln!("{} {}", "ERROR:".red(), format!($($arg)*))
	}
}