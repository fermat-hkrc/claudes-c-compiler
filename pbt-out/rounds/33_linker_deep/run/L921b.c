int g0 = 90643;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h2b(int);
typeedef int (*fp2)(int);
fp2 tbl2[2] = { 0, h2b };
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
int tent4;
int init_tent4() { tent4 = 33; return tent4; }
int tent5;
int init_tent5() { tent5 = 386; return tent5; }
