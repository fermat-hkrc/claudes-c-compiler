int g0 = -42563;
int tent1;
int init_tent1() { tent1 = 394; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 12) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 5) * 2; }
