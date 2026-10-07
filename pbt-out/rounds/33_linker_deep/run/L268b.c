int g0 = 39993;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep4(int);
int mid4(int x) { return deep4(x + -7) * 2; }
int tent5;
int init_tent5() { tent5 = 77; return tent5; }
