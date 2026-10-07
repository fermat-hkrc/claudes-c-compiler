int g0 = 64011;
extern int deep1(int);
int mid1(int x) { return deep1(x + -12) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -12) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
