int g0 = -77572;
extern int deep1(int);
int mid1(int x) { return deep1(x + 5) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -10) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
extern int deep5(int);
int mid5(int x) { return deep5(x + -3) * 2; }
