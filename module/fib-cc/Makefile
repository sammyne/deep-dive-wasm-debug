BUILD_DIR = build

fibcc = $(BUILD_DIR)/fib-cc.wasm

CXX = /opt/wasi-sdk/bin/clang++

.PHONY: all
all: $(fibcc)

$(BUILD_DIR):
	mkdir -p $@

$(fibcc): lib.cc $(BUILD_DIR)
	$(CXX) $< -g -o $@ -mexec-model=reactor --target=wasm32-unknown-wasi -std=c++20  -Wl,--no-entry -nostartfiles

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
