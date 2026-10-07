int g0 = -57018;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
int g3 = -5255;
extern int deep4(int);
int mid4(int x) { return deep4(x + 4) * 2; }
