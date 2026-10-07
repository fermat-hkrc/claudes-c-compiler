int g0 = -55088;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
int tent4;
int init_tent4() { tent4 = -478; return tent4; }
extern int deep5(int);
int mid5(int x) { return deep5(x + -2) * 2; }
