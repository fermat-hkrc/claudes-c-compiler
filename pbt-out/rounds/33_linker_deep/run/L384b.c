int g0 = 60408;
int tent1;
int init_tent1() { tent1 = -28; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 5) * 2; }
int tent3;
int init_tent3() { tent3 = 277; return tent3; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
