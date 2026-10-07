int g0 = 70915;
int g1 = -46375;
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep4(int);
int mid4(int x) { return deep4(x + -9) * 2; }
