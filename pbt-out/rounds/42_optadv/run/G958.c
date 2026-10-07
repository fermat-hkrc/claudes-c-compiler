static int f1(int *p, int *q) { *p += 426; return *q * 2; }
static int f2(int *p, int *q) { *p += 434; return *q * 2; }
static int f3(int *p, int *q) { *p += 788; return *q * 2; }
static int f4(int *p, int *q) { *p += 279; return *q * 2; }
static int f5(int *p, int *q) { *p += 933; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 143u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[2], &cells[7]);
    arr[0 % 6] = 36; cells[7] += arr[0 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f2(&cells[0], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[5] = cells[0] + 711;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[1] = cells[5] + 998;
    sum += f3(&cells[5], &cells[3]);
    sum += f4(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f5(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 780u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }