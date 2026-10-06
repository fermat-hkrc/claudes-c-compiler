struct F2 { double x, y; };              // 2 SSE eightbytes → xmm0/xmm1
double dot(struct F2 a, struct F2 b) { return a.x*b.x + a.y*b.y; }
int main(void) { struct F2 a = { 1.5, 2.5 }, b = { 4.0, 2.0 }; printf("%.17g\n", dot(a, b)); return 0; }
