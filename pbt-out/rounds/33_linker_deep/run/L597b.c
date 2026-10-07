int g0 = 82820;
int tent1;
int init_tent1() { tent1 = -394; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 6) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 5) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + 0) * 2; }
