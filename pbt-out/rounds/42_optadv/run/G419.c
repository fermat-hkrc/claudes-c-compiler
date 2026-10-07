static int f1(int *p, int *q) { *p += 588; return *q * 2; }
static int f2(int *p, int *q) { *p += 720; return *q * 2; }
static int f3(int *p, int *q) { *p += 42; return *q * 2; }
static int f4(int *p, int *q) { *p += 935; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[0]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[3], &cells[7]);
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 952u; }
    cells[7] = cells[0] + 104;
    cells[1] = cells[1] + 605;
    sum += f3(&cells[6], &cells[7]);
    sum += f4(&cells[3], &cells[4]);
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 262u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[6 % 6] = 337; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[2 % 6] = 755; cells[7] += arr[2 % 6];
    cells[7] = cells[7] + 68;
    arr[3 % 6] = 954; cells[5] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    arr[4 % 6] = 950; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }