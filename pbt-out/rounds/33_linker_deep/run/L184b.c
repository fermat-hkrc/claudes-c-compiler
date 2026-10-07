int g0 = 13169;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
extern int deep3(int);
int mid3(int x) { return deep3(x + 10) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 5) * 2; }
extern int deep5(int);
int mid5(int x) { return deep5(x + -2) * 2; }
