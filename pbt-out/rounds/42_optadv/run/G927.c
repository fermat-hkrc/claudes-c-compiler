static int f1(int *p, int *q) { *p += 64; return *q * 2; }
static int f2(int *p, int *q) { *p += 389; return *q * 2; }
static int f3(int *p, int *q) { *p += 794; return *q * 2; }
static int f4(int *p, int *q) { *p += 484; return *q * 2; }
static int f5(int *p, int *q) { *p += 312; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[6] + 214;
    arr[2 % 6] = 539; cells[5] += arr[2 % 6];
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += f1(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += f2(&cells[6], &cells[5]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 489u; }
    cells[1] = cells[4] + 378;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[3] + 584;
    cells[5] = cells[1] + 91;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[5], &cells[5]);
    sum += f4(&cells[7], &cells[7]);
    sum += f5(&cells[3], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }