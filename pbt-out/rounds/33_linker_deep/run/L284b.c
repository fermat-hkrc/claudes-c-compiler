int g0 = -23554;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep2(int);
int mid2(int x) { return deep2(x + 6) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
int tent4;
int init_tent4() { tent4 = 417; return tent4; }
extern int deep5(int);
int mid5(int x) { return deep5(x + 6) * 2; }
