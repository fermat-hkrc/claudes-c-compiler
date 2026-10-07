int g0 = 53333;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
int tent3;
int init_tent3() { tent3 = 458; return tent3; }
extern int deep5(int);
int mid5(int x) { return deep5(x + 8) * 2; }
