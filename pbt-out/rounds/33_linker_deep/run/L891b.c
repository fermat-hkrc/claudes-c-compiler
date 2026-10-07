int g0 = 65781;
extern int deep1(int);
int mid1(int x) { return deep1(x + 5) * 2; }
int tent2;
int init_tent2() { tent2 = -206; return tent2; }
