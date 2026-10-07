int g0 = -74285;
int g1 = -12597;
extern int deep2(int);
int mid2(int x) { return deep2(x + 3) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 1) * 2; }
int tent5;
int init_tent5() { tent5 = 366; return tent5; }
