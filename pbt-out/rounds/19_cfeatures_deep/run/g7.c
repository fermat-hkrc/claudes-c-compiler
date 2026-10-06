struct Bits { unsigned a : 5; unsigned b : 11; unsigned c : 16; };
int main(void) { struct Bits x = { 31, 2047, 65535 }; printf("%u %u %u %u\n", x.a, x.b, x.c, (unsigned)sizeof(x)); return 0; }
