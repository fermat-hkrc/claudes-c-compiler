int g0 = 4583;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep2(int);
int mid2(int x) { return deep2(x + 5) * 2; }
int tent3;
int init_tent3() { tent3 = 148; return tent3; }
extern int deep5(int);
int mid5(int x) { return deep5(x + 12) * 2; }
