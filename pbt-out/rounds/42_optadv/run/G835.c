static int f1(int *p, int *q) { *p += 96; return *q * 2; }
static int f2(int *p, int *q) { *p += 557; return *q * 2; }
static int f3(int *p, int *q) { *p += 584; return *q * 2; }
static int f4(int *p, int *q) { *p += 797; return *q * 2; }
static int f5(int *p, int *q) { *p += 259; return *q * 2; }
static int f6(int *p, int *q) { *p += 241; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[1] + 664;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 213u; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[5], &cells[2]);
    cells[4] = cells[7] + 297;
    cells[4] = cells[1] + 206;
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 499u; }
    sum += f3(&cells[2], &cells[0]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f4(&cells[4], &cells[4]);
    sum += f5(&cells[0], &cells[0]);
    sum += f6(&cells[3], &cells[7]);
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }