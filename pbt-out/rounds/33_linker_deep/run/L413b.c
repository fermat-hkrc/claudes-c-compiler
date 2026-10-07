int g0 = 42595;
extern int deep1(int);
int mid1(int x) { return deep1(x + 4) * 2; }
int tent3;
int init_tent3() { tent3 = 117; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -8) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
