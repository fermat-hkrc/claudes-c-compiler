int g0 = 37732;
extern int deep1(int);
int mid1(int x) { return deep1(x + -6) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 5) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 4) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
