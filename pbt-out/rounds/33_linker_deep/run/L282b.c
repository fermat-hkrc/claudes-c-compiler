int g0 = -61759;
int tent1;
int init_tent1() { tent1 = -282; return tent1; }
int tent2;
int init_tent2() { tent2 = -40; return tent2; }
int g3 = -76225;
extern int deep4(int);
int mid4(int x) { return deep4(x + 10) * 2; }
extern int h5b(int);
typeedef int (*fp5)(int);
fp5 tbl5[2] = { 0, h5b };
