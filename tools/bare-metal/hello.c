#include <stdint.h>

static volatile char *const uart = (char *)0x10000000;

static void print_string(const char *str) {
  while (*str)
    *uart = *str++;
}

void main(void) {
  print_string("Hello, World!\n");
  *(uint32_t *)0x100000 = 0x5555; // Shutdown
  for (;;)
    ;
}
