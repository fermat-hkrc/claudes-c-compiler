static int f1(int *p, int *q) { *p += 838; return *q * 2; }
static int f2(int *p, int *q) { *p += 597; return *q * 2; }
static int f3(int *p, int *q) { *p += 56; return *q * 2; }
static int f4(int *p, int *q) { *p += 341; return *q * 2; }
static int f5(int *p, int *q) { *p += 631; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[4], &cells[6]);
    arr[1 % 6] = 266; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 798u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 418u; }
    sum += f2(&cells[6], &cells[4]);
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 908u; }
    arr[1 % 6] = 817; cells[6] += arr[1 % 6];
    cells[3] = cells[7] + 995;
    sum += f3(&cells[0], &cells[7]);
    cells[0] = cells[6] + 989;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 5u; }
    sum += f4(&cells[5], &cells[0]);
    sum += f5(&cells[0], &cells[3]);
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 212u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }