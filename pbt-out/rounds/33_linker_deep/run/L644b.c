int g0 = 80473;
int g1 = 10779;
extern int deep2(int);
int mid2(int x) { return deep2(x + 8) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -8) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
