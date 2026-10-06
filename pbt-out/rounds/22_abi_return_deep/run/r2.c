struct DD { double x, y; };
struct DD rot(struct DD v) { struct DD r = { v.y, -v.x }; return r; }
int main(void) { struct DD v = rot((struct DD){ 1.5, 2.5 }), w = rot(v); printf("%.17g %.17g %.17g\n", v.x, v.y, w.x); return 0; }
