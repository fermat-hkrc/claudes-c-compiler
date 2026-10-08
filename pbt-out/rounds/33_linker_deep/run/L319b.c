int g0 = 36187;
int g1 = 35935;
int tent2;
int init_tent2() { tent2 = 407; return tent2; }
int tent3;
int init_tent3() { tent3 = 347; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 5) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
