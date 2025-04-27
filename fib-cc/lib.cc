#include <cstdint>

// export_name 之后才能找到调试符号。
__attribute__((export_name("fib")))
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