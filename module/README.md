# Debug module

## Environment
- wasmtime 32.0.0
- wasi-sdk 22.0

## Source codes
### fib-cc/lib.cc: C++ source codes
```c++
#include <cstdint>

__attribute__((__export_name__("fib")))
extern "C" uint32_t fib(uint32_t n) {
  uint32_t a = 1;
  uint32_t b = 1;

  for(int i=0; i<n; ++i) {
    auto t = a;
    a = b;
    b += t;
  }

  return b;
}
```

### debugger/src/main.rs: embedded wasmtime
```rust
use wasmtime::*;

fn main() -> Result<()> {
    let path = match std::env::args().skip(1).next() {
        Some(v) => v,
        None => "target/wasm32-unknown-unknown/debug/fib.wasm".to_owned(),
    };

    // Load our previously compiled wasm file (built previously with Cargo) and
    // also ensure that we generate debuginfo so this executable can be
    // debugged in GDB.
    let engine = Engine::new(
        Config::new()
            .debug_info(true)
            .cranelift_opt_level(OptLevel::None),
    )?;
    let mut store = Store::new(&engine, ());

    //const PATH: &str = "target/wasm32-wasip1/debug/fib.wasm";
    //const PATH: &str = "target/wasm32-unknown-unknown/debug/fib.wasm";

    let module = Module::from_file(&engine, path)?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // Invoke `fib` export
    let fib = instance.get_typed_func::<i32, i32>(&mut store, "fib")?;
    println!("fib(6) = {}", fib.call(&mut store, 6)?);
    Ok(())
}
```

### Makefile
```makefile
debugger = target/release/debugger
fibcc = fib-cc/build/fib-cc.wasm

.PHONY: all
all: $(debugger) $(fibcc)

$(debugger): debugger/src/main.rs
	cargo build -r -p debugger

$(fibcc): fib-cc/lib.cc
	$(MAKE) -C fib-cc

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
```

## Using export name as 'fib'

Running `make lldb-cc` with `__attribute__((__export_name__("fib")))` in fib-cc/lib.cc gives me expected output as

```bash
root@eafb8747aaf8:/workspace/module# make lldb-cc
cargo build -r -p debugger
   Compiling debugger v0.1.0 (/workspace/module/debugger)
    Finished `release` profile [optimized] target(s) in 1.36s
lldb -O 'settings set target.disable-aslr false' \
        -o 'breakpoint set -n main' \
        -o 'r' \
        -o "breakpoint set -n 'fib'" \
        -o 'c' \
        -o 'n' \
        -o 'p a' \
        -- target/release/debugger fib-cc/build/fib-cc.wasm
(lldb) settings set target.disable-aslr false
(lldb) target create "target/release/debugger"
Current executable set to '/workspace/module/target/release/debugger' (x86_64).
(lldb) settings set -- target.run-args  "fib-cc/build/fib-cc.wasm"
(lldb) breakpoint set -n main
Breakpoint 1: where = debugger`main, address = 0x00000000000baac0
(lldb) r
Process 93176 launched: '/workspace/module/target/release/debugger' (x86_64)
Process 93176 stopped
* thread #1, name = 'debugger', stop reason = breakpoint 1.1
    frame #0: 0x0000559dedb3aac0 debugger`main
debugger`main:
->  0x559dedb3aac0 <+0>: pushq  %rax
    0x559dedb3aac1 <+1>: movq   %rsi, %rcx
    0x559dedb3aac4 <+4>: movslq %edi, %rdx
    0x559dedb3aac7 <+7>: leaq   -0x59e(%rip), %rax ; debugger::main::h4a2646090ba600e9
(lldb) breakpoint set -n 'fib'
Breakpoint 2: no locations (pending).
WARNING:  Unable to resolve breakpoint to any actual locations.
(lldb) c
1 location added to breakpoint 2
Process 93176 resuming
Process 93176 stopped
* thread #1, name = 'debugger', stop reason = breakpoint 2.1
    frame #0: 0x00007ff04c07b013 JIT(0x559def806040)`fib(n=6) at lib.cc:6:12
   3    // export_name 之后才能找到调试符号。
   4    __attribute__((__export_name__("fib")))
   5    extern "C" uint32_t fib(uint32_t n) {
-> 6      uint32_t a = 1;
   7      uint32_t b = 1;
   8   
   9      for(int i=0; i<n; ++i) {
(lldb) n
Process 93176 stopped
* thread #1, name = 'debugger', stop reason = step over
    frame #0: 0x00007ff04c07b020 JIT(0x559def806040)`fib(n=6) at lib.cc:7:12
   4    __attribute__((__export_name__("fib")))
   5    extern "C" uint32_t fib(uint32_t n) {
   6      uint32_t a = 1;
-> 7      uint32_t b = 1;
   8   
   9      for(int i=0; i<n; ++i) {
   10       auto t = a;
(lldb) p a
(uint32_t) 1
```

## Problem: using export name as 'fib-cc' gives unexpected output

Running `make lldb-cc` with 
- `__attribute__((__export_name__("fib-cc")))` in fib-cc/lib.cc
- renaming target func as 'fib-cc' in debugger/src/main.rs
gives me unexpected output as

```bash
root@eafb8747aaf8:/workspace/module# make lldb-cc
lldb -O 'settings set target.disable-aslr false' \
        -o 'breakpoint set -n main' \
        -o 'r' \
        -o "breakpoint set -n 'fib-cc'" \
        -o 'c' \
        -o 'n' \
        -o 'p a' \
        -- target/release/debugger fib-cc/build/fib-cc.wasm
(lldb) settings set target.disable-aslr false
(lldb) target create "target/release/debugger"
Current executable set to '/workspace/module/target/release/debugger' (x86_64).
(lldb) settings set -- target.run-args  "fib-cc/build/fib-cc.wasm"
(lldb) breakpoint set -n main
Breakpoint 1: where = debugger`main, address = 0x00000000000baac0
(lldb) r
Process 103930 launched: '/workspace/module/target/release/debugger' (x86_64)
Process 103930 stopped
* thread #1, name = 'debugger', stop reason = breakpoint 1.1
    frame #0: 0x000055557f202ac0 debugger`main
debugger`main:
->  0x55557f202ac0 <+0>: pushq  %rax
    0x55557f202ac1 <+1>: movq   %rsi, %rcx
    0x55557f202ac4 <+4>: movslq %edi, %rdx
    0x55557f202ac7 <+7>: leaq   -0x59e(%rip), %rax ; debugger::main::h4a2646090ba600e9
(lldb) breakpoint set -n 'fib-cc'
Breakpoint 2: no locations (pending).
WARNING:  Unable to resolve breakpoint to any actual locations.
(lldb) c
fib(6) = 21
Process 103930 resuming
Process 103930 exited with status = -1 (0xffffffff) lost connection
(lldb) n
error: Command requires a process which is currently stopped.
```

Why LLDB cannot find symbol exported name 'fib-cc'?
