static int f1(int *p, int *q) { *p += 586; return *q * 2; }
static int f2(int *p, int *q) { *p += 776; return *q * 2; }
static int f3(int *p, int *q) { *p += 879; return *q * 2; }
static int f4(int *p, int *q) { *p += 590; return *q * 2; }
static int f5(int *p, int *q) { *p += 934; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 3; cells[0] += arr[3 % 6];
    arr[6 % 6] = 226; cells[2] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[3], &cells[4]);
    sum += f2(&cells[5], &cells[2]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 486u; }
    sum += f3(&cells[3], &cells[3]);
    cells[1] = cells[0] + 793;
    sum += f4(&cells[2], &cells[7]);
    sum += f5(&cells[4], &cells[2]);
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }