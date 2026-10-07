static int f1(int *p, int *q) { *p += 103; return *q * 2; }
static int f2(int *p, int *q) { *p += 874; return *q * 2; }
static int f3(int *p, int *q) { *p += 279; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 707;
    sum += f1(&cells[3], &cells[5]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 379u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[7] = cells[3] + 539;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += f2(&cells[0], &cells[3]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 466u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += f3(&cells[7], &cells[1]);
    cells[2] = cells[7] + 636;
    arr[7 % 6] = 678; cells[2] += arr[7 % 6];
    arr[0 % 6] = 275; cells[3] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }