int g0 = -67135;
extern int deep1(int);
int mid1(int x) { return deep1(x + -1) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + -4) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 11) * 2; }
int tent4;
int init_tent4() { tent4 = 269; return tent4; }
