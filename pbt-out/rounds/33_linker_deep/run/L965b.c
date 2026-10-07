int g0 = 15536;
int g1 = 1754;
extern int deep2(int);
int mid2(int x) { return deep2(x + 10) * 2; }
extern int deep3(int);
int mid3(int x) { return deep3(x + 9) * 2; }
int tent4;
int init_tent4() { tent4 = -459; return tent4; }
int tent5;
int init_tent5() { tent5 = 342; return tent5; }
