int g0 = 99359;
int tent1;
int init_tent1() { tent1 = -218; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -3) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 0) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -3) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
