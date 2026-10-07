int g0 = 68146;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
int tent2;
int init_tent2() { tent2 = 28; return tent2; }
int tent3;
int init_tent3() { tent3 = 267; return tent3; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 3) * 2; }
