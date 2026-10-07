int g0 = 42236;
extern int deep1(int);
int mid1(int x) { return deep1(x + -8) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + -10) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
