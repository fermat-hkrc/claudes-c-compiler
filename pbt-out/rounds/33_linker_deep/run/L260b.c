int g0 = 17483;
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
int tent3;
int init_tent3() { tent3 = 111; return tent3; }
extern int deep5(int);
int mid5(int x) { return deep5(x + 4) * 2; }
