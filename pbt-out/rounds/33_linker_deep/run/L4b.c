int g0 = -27559;
extern int deep1(int);
int mid1(int x) { return deep1(x + 7) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 11) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 0) * 2; }
int tent4;
int init_tent4() { tent4 = -196; return tent4; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
