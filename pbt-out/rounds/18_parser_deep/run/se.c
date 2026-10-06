int f(void) { return sizeof(enum { X, Y }) * 10 + (int)Y; }
int main(void){return 0;}
