static int f1(int *p, int *q) { *p += 913; return *q * 2; }
static int f2(int *p, int *q) { *p += 893; return *q * 2; }
static int f3(int *p, int *q) { *p += 238; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    arr[2 % 6] = 43; cells[2] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 850u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 458u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 250u; }
    sum += f1(&cells[6], &cells[7]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 167u; }
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 176u; }
    arr[7 % 6] = 261; cells[5] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 500u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += f2(&cells[3], &cells[0]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[7 % 6] = 87; cells[7] += arr[7 % 6];
    sum += f3(&cells[2], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }