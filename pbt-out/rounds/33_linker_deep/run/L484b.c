int g0 = -57669;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
int tent3;
int init_tent3() { tent3 = 10; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -5) * 2; }
