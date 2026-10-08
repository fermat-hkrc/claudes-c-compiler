int g0 = 45377;
extern int deep1(int);
int mid1(int x) { return deep1(x + 4) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 12) * 2; }
int tent3;
int init_tent3() { tent3 = 195; return tent3; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + -9) * 2; }
