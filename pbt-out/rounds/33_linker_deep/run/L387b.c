int g0 = 68486;
int tent1;
int init_tent1() { tent1 = -359; return tent1; }
int tent2;
int init_tent2() { tent2 = -184; return tent2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 5) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 2) * 2; }
