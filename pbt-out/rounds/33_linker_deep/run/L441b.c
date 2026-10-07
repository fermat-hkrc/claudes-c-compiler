int g0 = -17177;
int tent1;
int init_tent1() { tent1 = 45; return tent1; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 3) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep4(int);
int mid4(int x) { return deep4(x + 11) * 2; }
