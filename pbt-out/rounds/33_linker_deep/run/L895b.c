int g0 = 19791;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
int tent2;
int init_tent2() { tent2 = 130; return tent2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 9) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 0) * 2; }
extern int deep5(int);
int mid5(int x) { return deep5(x + -11) * 2; }
