struct C16 { char b[16]; };
struct C16 tag(struct C16 v) { v.b[0]++; v.b[15] = 'Z'; return v; }
int main(void) { struct C16 v = { { 'a' } }; struct C16 w = tag(v); printf("%c%c %d\n", w.b[0], w.b[15], v.b[0]); return 0; }
