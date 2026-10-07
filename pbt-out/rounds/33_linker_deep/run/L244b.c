int g0 = -58030;
extern int deep1(int);
int mid1(int x) { return deep1(x + -6) * 2; }
int tent2;
int init_tent2() { tent2 = 210; return tent2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 10) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
