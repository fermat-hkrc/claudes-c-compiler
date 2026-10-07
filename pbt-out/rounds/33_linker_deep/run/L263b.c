int g0 = -42823;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep2(int);
int mid2(int x) { return deep2(x + -3) * 2; }
int tent3;
int init_tent3() { tent3 = -192; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -3) * 2; }
int tent5;
int init_tent5() { tent5 = 92; return tent5; }
