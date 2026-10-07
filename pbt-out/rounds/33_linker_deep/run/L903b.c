int g0 = 64999;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep4(int);
int mid4(int x) { return deep4(x + 10) * 2; }
int tent5;
int init_tent5() { tent5 = 346; return tent5; }
