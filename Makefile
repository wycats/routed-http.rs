default:
	cd src && rustc -L ../lib --opt-level=3 main.rs
