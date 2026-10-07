int g0 = 55656;
extern int deep1(int);
int mid1(int x) { return deep1(x + 4) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -5) * 2; }
int tent3;
int init_tent3() { tent3 = 421; return tent3; }
int tent4;
int init_tent4() { tent4 = -149; return tent4; }
