int g0 = 22813;
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
extern int deep3(int);
int mid3(int x) { return deep3(x + 7) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 6) * 2; }
int tent5;
int init_tent5() { tent5 = 471; return tent5; }
