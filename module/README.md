# Debug module

## 1. Environment
- wasmtime 32.0.0
- wasi-sdk 22.0
- LLDB 18.1.8

## 2. Source codes
### fib-cc/lib.cc: C++ source codes
```c++
#include <cstdint>

__attribute__((__export_name__("fib")))
extern "C" uint32_t fib(uint32_t n) {
  uint32_t a = 1;
  uint32_t b = 1;

  hello h{
    .ptr = (const uint8_t*)"hello world",
    .len = sizeof("hello world"),
  };

  for(int i=0; i<n; ++i) {
    auto t = a;
    a = b;
    b += t;
  }

  return b + h.ptr[1];
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

## 3. Problem reproduction

```bash
make lldb-cc
```

Logs go as
```bash
root@bd04aefd8e7c:/workspace/module# make lldb-cc
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
        -- target/release/debugger fib-cc/build/fib-cc.wasm
(lldb) settings set target.disable-aslr false
(lldb) target create "target/release/debugger"
Current executable set to '/workspace/module/target/release/debugger' (x86_64).
(lldb) settings set -- target.run-args  "fib-cc/build/fib-cc.wasm"
(lldb) breakpoint set -n main
Breakpoint 1: where = debugger`main, address = 0x00000000000baac0
(lldb) r
Process 5041 launched: '/workspace/module/target/release/debugger' (x86_64)
Process 5041 stopped
* thread #1, name = 'debugger', stop reason = breakpoint 1.1
    frame #0: 0x0000557f8015aac0 debugger`main
debugger`main:
->  0x557f8015aac0 <+0>: pushq  %rax
    0x557f8015aac1 <+1>: movq   %rsi, %rcx
    0x557f8015aac4 <+4>: movslq %edi, %rdx
    0x557f8015aac7 <+7>: leaq   -0x59e(%rip), %rax ; debugger::main::h4a2646090ba600e9
(lldb) breakpoint set -n 'fib'
Breakpoint 2: no locations (pending).
WARNING:  Unable to resolve breakpoint to any actual locations.
(lldb) c
1 location added to breakpoint 2
Process 5041 resuming
Process 5041 stopped
* thread #1, name = 'debugger', stop reason = breakpoint 2.1
    frame #0: 0x00007fcd740f4015 JIT(0x557f812a0590)`fib(n=6) at lib.cc:13:12
   10   // 调试符号应该用 C 的函数名，而不是 export-name。
   11   __attribute__((__export_name__("fib-cc")))
   12   extern "C" uint32_t fib(uint32_t n) {
-> 13     uint32_t a = 1;
   14     uint32_t b = 1;
   15  
   16     hello h{.ptr = (const uint8_t*)"hello world", .len = sizeof("hello world")};
(lldb) n
Process 5041 stopped
* thread #1, name = 'debugger', stop reason = step over
    frame #0: 0x00007fcd740f4022 JIT(0x557f812a0590)`fib(n=6) at lib.cc:14:12
   11   __attribute__((__export_name__("fib-cc")))
   12   extern "C" uint32_t fib(uint32_t n) {
   13     uint32_t a = 1;
-> 14     uint32_t b = 1;
   15  
   16     hello h{.ptr = (const uint8_t*)"hello world", .len = sizeof("hello world")};
   17
(lldb) n
Process 5041 stopped
* thread #1, name = 'debugger', stop reason = step over
    frame #0: 0x00007fcd740f402f JIT(0x557f812a0590)`fib(n=6) at lib.cc:16:9
   13     uint32_t a = 1;
   14     uint32_t b = 1;
   15  
-> 16     hello h{.ptr = (const uint8_t*)"hello world", .len = sizeof("hello world")};
   17  
   18     for(int i=0; i<n; ++i) {
   19       auto t = a;
(lldb) n
Process 5041 stopped
* thread #1, name = 'debugger', stop reason = step over
    frame #0: 0x00007fcd740f4069 JIT(0x557f812a0590)`fib(n=6) at lib.cc:18:11
   15  
   16     hello h{.ptr = (const uint8_t*)"hello world", .len = sizeof("hello world")};
   17  
-> 18     for(int i=0; i<n; ++i) {
   19       auto t = a;
   20       a = b;
   21       b += t;
(lldb) expr (void)__vmctx->set()
(lldb) p *h.ptr
(const uint8_t) 'h'
(lldb) expr *(char(*)[12])(&(__vmctx.memory[1024]))
(char[12]) $0 = "hello world"
  Evaluated this expression after applying Fix-It(s):
    *(char(*)[12])(&(__vmctx->memory[1024]))
(lldb) p *(char(*)[12])(&h.ptr)
(char[12]) "\0\U00000004\0\0\0\0\0\0\f"
(lldb) p *(char(*)[12])(h.ptr)
error: <user expression 5>:1:2: cannot cast from type 'WebAssemblyPtrWrapper<const uint8_t>' to pointer type 'char (*)[12]'
    1 | *(char(*)[12])(h.ptr)
      |  ^~~~~~~~~~~~~~~~~~~~
```

The log indicates that `p *(char(*)[12])(h.ptr)` failed to convert `h.ptr` as a readable c-str of 12 char (which it actually is).

Two questions:
1. How can I convert `WebAssemblyPtrWrapper<const uint8_t>` as `*(char(*)[12])` for printing?
2. How to resolve failure to set breakpoint on 'fib' symbol with errors as
    ```bash
    Breakpoint 2: no locations (pending).
    WARNING:  Unable to resolve breakpoint to any actual locations.
    ```
