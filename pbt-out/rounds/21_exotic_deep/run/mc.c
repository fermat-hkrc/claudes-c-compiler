#include <string.h>
struct S { char b[16]; };
struct S mk(const char *s) { struct S r; memcpy(r.b, s, 16); memset(r.b + 8, 'x', 8); return r; }
int main(void) {
  char buf[16] = { 0 };
  memcpy(buf, "hello", 6);
  struct S s = mk(buf);
  printf("%s|%c%c %d\n", s.b, s.b[8], s.b[15], (int)strlen(buf));
  return 0;
}
