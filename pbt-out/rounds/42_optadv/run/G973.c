static int f1(int *p, int *q) { *p += 528; return *q * 2; }
static int f2(int *p, int *q) { *p += 412; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[2] + 481;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 322u; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[2] = cells[0] + 606;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 838u; }
    sum += f1(&cells[4], &cells[1]);
    cells[4] = cells[7] + 569;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 190u; }
    sum += f2(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[3 % 6] = 595; cells[1] += arr[3 % 6];
    arr[3 % 6] = 418; cells[4] += arr[3 % 6];
    arr[6 % 6] = 723; cells[6] += arr[6 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 659u; }
    arr[6 % 6] = 324; cells[2] += arr[6 % 6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 248u; }
    arr[4 % 6] = 682; cells[3] += arr[4 % 6];
    cells[1] = cells[6] + 559;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }