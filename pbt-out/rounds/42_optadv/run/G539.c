static int f1(int *p, int *q) { *p += 176; return *q * 2; }
static int f2(int *p, int *q) { *p += 215; return *q * 2; }
static int f3(int *p, int *q) { *p += 955; return *q * 2; }
static int f4(int *p, int *q) { *p += 978; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 639; cells[1] += arr[4 % 6];
    sum += f1(&cells[2], &cells[2]);
    cells[1] = cells[7] + 56;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[4], &cells[4]);
    arr[3 % 6] = 254; cells[2] += arr[3 % 6];
    cells[7] = cells[1] + 970;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 951u; }
    arr[6 % 6] = 13; cells[0] += arr[6 % 6];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 78u; }
    sum += f3(&cells[7], &cells[0]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 656u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[7] = cells[7] + 533;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 723u; }
    sum += f4(&cells[4], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }