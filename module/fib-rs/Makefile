#app = target/wasm32-wasip1/debug/fib.wasm
app = target/wasm32-unknown-unknown/debug/fib.wasm

.PHONY: all
all: $(app)

$(app): src/lib.rs
#	cargo build --target=wasm32-wasip1
	cargo build --target=wasm32-unknown-unknown

lldb: $(app)
	lldb -O 'settings set target.disable-aslr false' -- wasmtime run -D debug-info -O opt-level=0 $<

gdb: $(app)
	gdb --args wasmtime run -D debug-info -O opt-level=0 $<

debug: $(app)
	../debugger/target/release/debugger

lldb-debug: $(app)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-- ../debugger/target/release/debugger

lldb-debug-v2: $(app)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-- ./debugger

run: $(app)
	wasmtime $(app)

.PHONY: clean
clean:
	cargo clean