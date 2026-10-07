int g0 = 8317;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + -5) * 2; }
