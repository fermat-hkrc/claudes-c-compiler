int g0 = -71416;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep2(int);
int mid2(int x) { return deep2(x + 1) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + -10) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 0) * 2; }
int tent5;
int init_tent5() { tent5 = 235; return tent5; }
