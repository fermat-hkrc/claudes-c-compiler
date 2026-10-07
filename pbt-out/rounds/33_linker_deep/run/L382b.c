int g0 = 10124;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep2(int);
int mid2(int x) { return deep2(x + -9) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + -7) * 2; }
extern int deep5(int);
int mid5(int x) { return deep5(x + 0) * 2; }
