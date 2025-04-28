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
		-o 'n' \
		-o 'n' \
		-o 'expr (void)__vmctx->set()' \
		-o 'p *h.ptr' \
		-o 'expr *(char(*)[12])(&(__vmctx.memory[1024]))' \
		-o 'p *(char(*)[12])(&h.ptr)' \
		-o 'p *(char(*)[12])(h.ptr)' \
		-- $(debugger) $(fibcc)

.PHONY: clean
clean:
	cargo clean
	$(MAKE) -C fib-cc clean