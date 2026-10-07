int g0 = -38821;
int tent1;
int init_tent1() { tent1 = 210; return tent1; }
int tent2;
int init_tent2() { tent2 = -132; return tent2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 4) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
