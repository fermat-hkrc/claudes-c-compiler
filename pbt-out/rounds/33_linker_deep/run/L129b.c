int g0 = 24266;
extern int deep1(int);
int mid1(int x) { return deep1(x + 11) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 11) * 2; }
int g3 = 63254;
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
int tent5;
int init_tent5() { tent5 = 71; return tent5; }
