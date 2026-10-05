#define VAC(fmt, ...) g(fmt, ## __VA_ARGS__)
VAC(a, )
VAC(a)
VAC(a, b)
