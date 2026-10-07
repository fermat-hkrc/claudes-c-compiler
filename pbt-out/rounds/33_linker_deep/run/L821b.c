int g0 = -53444;
int tent1;
int init_tent1() { tent1 = 250; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 0) * 2; }
int tent3;
int init_tent3() { tent3 = 269; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -6) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
