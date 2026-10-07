int g0 = 81416;
extern int deep2(int);
int mid2(int x) { return deep2(x + 9) * 2; }
int tent3;
int init_tent3() { tent3 = 490; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -9) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
