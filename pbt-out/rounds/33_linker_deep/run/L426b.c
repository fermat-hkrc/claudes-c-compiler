int g0 = -20466;
extern int deep1(int);
int mid1(int x) { return deep1(x + 1) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -7) * 2; }
int tent3;
int init_tent3() { tent3 = 120; return tent3; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
