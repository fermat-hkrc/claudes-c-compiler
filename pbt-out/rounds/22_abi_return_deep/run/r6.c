struct F4 { float a, b, c, d; };
struct F4 shuffle(struct F4 v) { struct F4 r = { v.d, v.a, v.b, v.c }; return r; }
int main(void) { struct F4 v = shuffle((struct F4){ 1, 2, 3, 4 }), w = shuffle(v); printf("%.1f %.1f %.1f\n", v.a, v.d, w.b); return 0; }
