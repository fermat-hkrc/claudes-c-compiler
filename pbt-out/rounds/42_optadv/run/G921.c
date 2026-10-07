static int f1(int *p, int *q) { *p += 779; return *q * 2; }
static int f2(int *p, int *q) { *p += 983; return *q * 2; }
static int f3(int *p, int *q) { *p += 126; return *q * 2; }
static int f4(int *p, int *q) { *p += 470; return *q * 2; }
static int f5(int *p, int *q) { *p += 782; return *q * 2; }
static int f6(int *p, int *q) { *p += 145; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[3]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[4] = cells[4] + 896;
    arr[5 % 6] = 1; cells[7] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 880u; }
    sum += f2(&cells[6], &cells[3]);
    cells[2] = cells[6] + 219;
    cells[7] = cells[7] + 215;
    sum += f3(&cells[0], &cells[3]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += f4(&cells[6], &cells[5]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 463u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 288u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += f5(&cells[0], &cells[0]);
    arr[6 % 6] = 934; cells[0] += arr[6 % 6];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 609u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f6(&cells[2], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }