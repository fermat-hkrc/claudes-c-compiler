#define E() 42
#define V(...) __VA_ARGS__ + 0
int a = E();
int b = V();
int c = V(1,2);
