int g0 = -9650;
int g1 = -57111;
extern int deep2(int);
int mid2(int x) { return deep2(x + -11) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + -7) * 2; }
int tent4;
int init_tent4() { tent4 = -330; return tent4; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
