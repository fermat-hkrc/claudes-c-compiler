int g0 = 73189;
int tent1;
int init_tent1() { tent1 = 20; return tent1; }
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 1) * 2; }
