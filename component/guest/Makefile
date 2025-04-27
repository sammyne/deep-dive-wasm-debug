
OUT_DIR := out
BINDGEN_DIR := $(OUT_DIR)/bindgen

WASIP2_ADAPTER_PATH := /opt/wasmtime/adapter-modules/wasi_snapshot_preview1.reactor.wasm
WORLD_NAME := helloworld

CC := /opt/wasi-sdk/bin/clang++
CCFLAGS := -std=c++20 -g -fno-exceptions -Wall -I$(BINDGEN_DIR)
LDFLAGS := -mexec-model=reactor

all: $(OUT_DIR)/app.wasm

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

$(OUT_DIR)/app.wasm: $(OUT_DIR)/greeter.wasm
	wasm-tools component new $< --adapt $(WASIP2_ADAPTER_PATH) -o $@

$(OUT_DIR)/greeter.wasm: src/lib.cc $(BINDGEN_DIR) $(OUT_DIR)
	$(CC) $(CCFLAGS) $< $(BINDGEN_DIR)/$(WORLD_NAME).c $(BINDGEN_DIR)/$(WORLD_NAME)_component_type.o -o $@ -mexec-model=reactor

$(BINDGEN_DIR): wit/world.wit $(OUT_DIR)
	@rm -rf $@
	@mkdir $@
	@wit-bindgen c --out-dir $@ $<

.PHONY: clean
clean:
	rm -rf $(BINDGEN_DIR) $(OUT_DIR)
