int g0 = -9173;
extern int deep1(int);
int mid1(int x) { return deep1(x + 0) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 4) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -8) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
