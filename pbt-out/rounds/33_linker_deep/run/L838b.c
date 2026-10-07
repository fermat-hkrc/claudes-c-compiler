int g0 = 45811;
int g1 = 31325;
int tent2;
int init_tent2() { tent2 = 200; return tent2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 11) * 2; }
extern int deep5(int);
int mid5(int x) { return deep5(x + -10) * 2; }
