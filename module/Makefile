debugger = target/release/debugger
fibrs = target/wasm32-unknown-unknown/debug/fib.wasm

fibcc = fib-cc/build/fib-cc.wasm

.PHONY: all
all: $(debugger) $(fibrs) $(fibcc)

$(debugger): debugger/src/main.rs
	cargo build -r -p debugger

$(fibrs): fib-rs/src/lib.rs
	cargo build -p fib --target=wasm32-unknown-unknown

$(fibcc): fib-cc/lib.cc
	$(MAKE) -C fib-cc

lldb-rs: $(debugger) $(fibrs)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-o 'breakpoint set -n fib' \
		-o 'c' \
		-o 'n' \
		-o 'p a' \
		-- $(debugger) $(fibrs)

lldb-cc: $(debugger) $(fibcc)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-o "breakpoint set -n 'fib'" \
		-o 'c' \
		-o 'n' \
		-o 'p a' \
		-- $(debugger) $(fibcc)

.PHONY: clean
clean:
	cargo clean
	$(MAKE) -C fib-cc clean