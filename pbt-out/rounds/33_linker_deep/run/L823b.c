int g0 = -67493;
int g1 = 99607;
extern int deep3(int);
int mid3(int x) { return deep3(x + -6) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 11) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
