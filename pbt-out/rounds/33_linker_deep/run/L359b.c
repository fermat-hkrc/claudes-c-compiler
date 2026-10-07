int g0 = 7808;
extern int deep1(int);
int mid1(int x) { return deep1(x + 6) * 2; }
int g2 = 57301;
extern int deep3(int);
int mid3(int x) { return deep3(x + 5) * 2; }
extern int h4b(int);
typeedef int (*fp4)(int);
fp4 tbl4[2] = { 0, h4b };
