#define EMPTY
#define ID(x) x
#define CATV(a,b) a##b
#define TWO(a,b) ((a)+(b))
#define VAF(fmt, ...) f(fmt, __VA_ARGS__)
#define VAC(fmt, ...) g(fmt, ## __VA_ARGS__)
#define OBJ 7
VAC(8, )
