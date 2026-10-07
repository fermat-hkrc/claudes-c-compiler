int g0 = 5473;
extern int deep2(int);
int mid2(int x) { return deep2(x + -10) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + -10) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
int tent5;
int init_tent5() { tent5 = -322; return tent5; }
