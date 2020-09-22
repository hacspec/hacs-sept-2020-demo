HACSPEC_HOME?=../hacspec
DEMO_HOME=$(PWD)

build:
	cargo +nightly build

HACPSEC=cd $(HACSPEC_HOME)/language && cargo +nightly run -- \
    -L $(DEMO_HOME)/target/debug/deps \
    --crate-type=lib --edition=2018 \
    --extern=hacspec_lib

typecheck_poly1305: build 
	$(HACPSEC) -Zno-codegen \
    	$(DEMO_HOME)/hacspec-examples/poly1305/poly1305.rs

translate_poly1305: build 
	$(HACPSEC) -o $(DEMO_HOME)/hacspec-examples/poly1305/Hacspec.Poly1305-bug.fst \
    	$(DEMO_HOME)/hacspec-examples/poly1305/poly1305.rs


typecheck_poly_demo: build 
	$(HACPSEC) -Zno-codegen \
    	$(DEMO_HOME)/hacspec-examples/poly-demo/src/lib.rs

typecheck_chacha20: build
	$(HACPSEC) -Zno-codegen \
    	$(DEMO_HOME)/hacspec-examples/chacha20/chacha20.rs

typecheck_chacha20poly1305: build 
	$(HACPSEC) -Zno-codegen \
	    --extern=chacha20 --extern=poly1305 \
    	$(DEMO_HOME)/hacspec-examples/chacha20poly1305/src/chacha20poly1305.rs 

typecheck_chacha20poly1305_rfc7539: build 
	$(HACPSEC) -Zno-codegen \
		--extern=chacha20 --extern=poly1305 \
    	$(DEMO_HOME)/hacspec-examples/chacha20poly1305-rfc7539/src/chacha20poly1305.rs 

all: typecheck_poly1305 typecheck_poly_demo typecheck_chacha20 \
	typecheck_chacha20poly1305 typecheck_chacha20poly1305_rfc7539
