int g0 = -77851;
extern int deep1(int);
int mid1(int x) { return deep1(x + 4) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 8) * 2; }
int tent3;
int init_tent3() { tent3 = -312; return tent3; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
