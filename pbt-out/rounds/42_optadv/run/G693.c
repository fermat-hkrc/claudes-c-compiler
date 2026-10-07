static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 687; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    arr[1 % 6] = 205; cells[5] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f2(int *p, int *q) { *p += 816; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 782u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[3 % 6] = 972; cells[1] += arr[3 % 6];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 228u; }
    static int f3(int *p, int *q) { *p += 154; return *q * 2; }
    sum += f3(&cells[3], &cells[5]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    static int f4(int *p, int *q) { *p += 45; return *q * 2; }
    sum += f4(&cells[0], &cells[5]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }