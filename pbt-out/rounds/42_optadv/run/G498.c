static int f1(int *p, int *q) { *p += 351; return *q * 2; }
static int f2(int *p, int *q) { *p += 732; return *q * 2; }
static int f3(int *p, int *q) { *p += 989; return *q * 2; }
static int f4(int *p, int *q) { *p += 347; return *q * 2; }
static int f5(int *p, int *q) { *p += 314; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[1] + 475;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += f1(&cells[3], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[7 % 6] = 537; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[0] + 145;
    sum += f2(&cells[7], &cells[5]);
    cells[6] = cells[1] + 920;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    sum += f3(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += f4(&cells[0], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f5(&cells[0], &cells[0]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 205u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }