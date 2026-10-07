int g0 = 53456;
int g1 = -66095;
extern int deep2(int);
int mid2(int x) { return deep2(x + -3) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 11) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 12) * 2; }
