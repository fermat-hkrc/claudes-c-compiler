int g0 = -90798;
extern int deep1(int);
int mid1(int x) { return deep1(x + 7) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 0) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
