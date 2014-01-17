LIB_DIR = ./lib
BUILD_DIR = ./build

all: routed-http examples

routed-http:
	rustc -L $(LIB_DIR) --opt-level=3 src/routed_http/lib.rs --out-dir $(BUILD_DIR)

examples:
	rustc -L $(LIB_DIR) -L $(BUILD_DIR) --opt-level=3 src/examples/main.rs --out-dir $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)/*

.PHONY: all routed-http examples clean