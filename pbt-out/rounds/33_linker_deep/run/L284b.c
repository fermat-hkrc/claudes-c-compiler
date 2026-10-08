int g0 = 2486;
extern int deep1(int);
int mid1(int x) { return deep1(x + -10) * 2; }
int tent2;
int init_tent2() { tent2 = -45; return tent2; }
int tent3;
int init_tent3() { tent3 = -280; return tent3; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
