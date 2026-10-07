int g0 = -26459;
extern int deep1(int);
int mid1(int x) { return deep1(x + -10) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -12) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
