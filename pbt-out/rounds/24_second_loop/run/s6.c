typedef int T;
int a[3]; int b[4]; void f2(void){ extern int a[]; int x = sizeof(a); (void)x; }
int main(void){return 0;}
