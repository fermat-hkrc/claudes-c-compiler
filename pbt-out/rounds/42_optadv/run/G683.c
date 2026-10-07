static int f1(int *p, int *q) { *p += 224; return *q * 2; }
static int f2(int *p, int *q) { *p += 327; return *q * 2; }
static int f3(int *p, int *q) { *p += 124; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 710u; }
    cells[7] = cells[5] + 493;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[0 % 6] = 442; cells[3] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[2] + 580;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 874u; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[7], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += f2(&cells[2], &cells[6]);
    arr[2 % 6] = 263; cells[3] += arr[2 % 6];
    sum += f3(&cells[4], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }