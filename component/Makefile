.PHONY: all
all: run

build-guest: guest/out/app.wasm

build-host:
	cd host-rs && cargo build -r

guest/out/app.wasm:
	make -C guest

lldb: guest/out/app.wasm build-host
# 	lldb -- host-rs/target/release/host -p $< -n sammyne
# lldb-cc: $(debugger) $(fibcc)
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-o 'breakpoint set -n say-hello-again' \
		-o 'c' \
		-o 'n' \
		-o 'p a' \
		-- host-rs/target/release/host -p $< -n 'alice'

lldb-scratch: guest/out/app.wasm build-host
	lldb -O 'settings set target.disable-aslr false' \
		-o 'breakpoint set -n main' \
		-o 'r' \
		-- host-rs/target/release/host -p $< -n 'alice'

run: guest/out/app.wasm build-host
	host-rs/target/release/host -p $< -n sammyne

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

.PHONY: clean
clean:
	make -C guest clean
	cd host-rs && cargo clean
