static int f1(int *p, int *q) { *p += 847; return *q * 2; }
static int f2(int *p, int *q) { *p += 376; return *q * 2; }
static int f3(int *p, int *q) { *p += 674; return *q * 2; }
static int f4(int *p, int *q) { *p += 121; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[7] + 368;
    arr[5 % 6] = 893; cells[1] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 219u; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 665u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[4 % 6] = 101; cells[2] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[1 % 6] = 983; cells[1] += arr[1 % 6];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 862u; }
    cells[5] = cells[4] + 804;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 392u; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 756u; }
    sum += f1(&cells[3], &cells[3]);
    sum += f2(&cells[1], &cells[7]);
    cells[7] = cells[6] + 639;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f3(&cells[2], &cells[2]);
    sum += f4(&cells[0], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }