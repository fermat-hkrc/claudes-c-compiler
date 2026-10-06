struct Big { long a[5]; };                // 40B struct → MEMORY class (stack)
struct Big mk(long s) { struct Big b; for (int i = 0; i < 5; i++) b.a[i] = s * (i + 1); return b; }
long tot(struct Big b) { long t = 0; for (int i = 0; i < 5; i++) t += b.a[i]; return t; }
int main(void) { printf("%ld\n", tot(mk(3))); return 0; }
