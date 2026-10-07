int g0 = -28986;
int tent1;
int init_tent1() { tent1 = 314; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 8) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
