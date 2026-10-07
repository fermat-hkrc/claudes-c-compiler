int g0 = 59650;
extern int deep1(int);
int mid1(int x) { return deep1(x + 6) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 3) * 2; }
int tent5;
int init_tent5() { tent5 = -375; return tent5; }
