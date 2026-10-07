static int f1(int *p, int *q) { *p += 619; return *q * 2; }
static int f2(int *p, int *q) { *p += 668; return *q * 2; }
static int f3(int *p, int *q) { *p += 619; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 327u; }
    cells[7] = cells[3] + 454;
    sum += f1(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += f2(&cells[4], &cells[3]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f3(&cells[3], &cells[5]);
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 68u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 321u; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 705; cells[3] += arr[0 % 6];
    cells[2] = cells[7] + 61;
    arr[5 % 6] = 704; cells[4] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }