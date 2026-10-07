int g0 = -86632;
int g1 = 9895;
extern int deep3(int);
int mid3(int x) { return deep3(x + 8) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + 6) * 2; }
int tent5;
int init_tent5() { tent5 = 15; return tent5; }
