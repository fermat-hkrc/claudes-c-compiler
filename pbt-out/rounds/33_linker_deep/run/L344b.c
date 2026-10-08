int g0 = -22403;
int g1 = 22874;
extern int deep2(int);
int mid2(int x) { return deep2(x + -3) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 9) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
