static int f1(int *p, int *q) { *p += 574; return *q * 2; }
static int f2(int *p, int *q) { *p += 425; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[7] = cells[5] + 210;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[3 % 6] = 137; cells[4] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[6] + 712;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    arr[5 % 6] = 665; cells[5] += arr[5 % 6];
    cells[7] = cells[7] + 394;
    arr[7 % 6] = 63; cells[7] += arr[7 % 6];
    sum += f1(&cells[6], &cells[5]);
    cells[0] = cells[1] + 889;
    sum += f2(&cells[0], &cells[1]);
    arr[7 % 6] = 482; cells[6] += arr[7 % 6];
    arr[4 % 6] = 319; cells[6] += arr[4 % 6];
    arr[0 % 6] = 306; cells[5] += arr[0 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 729u; }
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 345u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }