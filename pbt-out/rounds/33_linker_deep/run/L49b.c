int g0 = -42213;
int tent1;
int init_tent1() { tent1 = -214; return tent1; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 9) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
int tent5;
int init_tent5() { tent5 = 381; return tent5; }
