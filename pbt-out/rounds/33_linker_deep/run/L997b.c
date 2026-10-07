int g0 = -12658;
extern int deep1(int);
int mid1(int x) { return deep1(x + 9) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 3) * 2; }
int tent3;
int init_tent3() { tent3 = -447; return tent3; }
