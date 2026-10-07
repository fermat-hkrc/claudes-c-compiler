static int f1(int *p, int *q) { *p += 481; return *q * 2; }
static int f2(int *p, int *q) { *p += 336; return *q * 2; }
static int f3(int *p, int *q) { *p += 502; return *q * 2; }
static int f4(int *p, int *q) { *p += 551; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 146u; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 635u; }
    sum += f1(&cells[4], &cells[6]);
    cells[2] = cells[1] + 247;
    arr[5 % 6] = 412; cells[4] += arr[5 % 6];
    cells[4] = cells[0] + 401;
    cells[0] = cells[6] + 125;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[3], &cells[5]);
    sum += f3(&cells[0], &cells[1]);
    cells[6] = cells[5] + 79;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    cells[4] = cells[2] + 979;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    cells[1] = cells[5] + 691;
    cells[5] = cells[0] + 606;
    sum += f4(&cells[2], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }