#include <cstdint>

struct hello {
  const uint8_t *ptr;
  uint64_t len; 
};

// __export_name__ 之后才能找到调试符号。
// __export_name__ 也可以写为 export_name，建议用 export_name。
// 调试符号应该用 C 的函数名，而不是 export-name。
__attribute__((__export_name__("fib-cc")))
extern "C" uint32_t fib(uint32_t n) {
  uint32_t a = 1;
  uint32_t b = 1;

  hello h{.ptr = (const uint8_t*)"hello world", .len = sizeof("hello world")};

  for(int i=0; i<n; ++i) {
    auto t = a;
    a = b;
    b += t;
  }

  return b + h.ptr[1];
}