int g0 = -42976;
extern int deep1(int);
int mid1(int x) { return deep1(x + -5) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -3) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
int g4 = -92044;
