static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f1(int *p, int *q) { *p += 889; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 410u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f2(int *p, int *q) { *p += 205; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    static int f3(int *p, int *q) { *p += 10; return *q * 2; }
    sum += f3(&cells[5], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }