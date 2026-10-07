int g0 = -69810;
int tent1;
int init_tent1() { tent1 = -81; return tent1; }
int tent3;
int init_tent3() { tent3 = -120; return tent3; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 8) * 2; }
