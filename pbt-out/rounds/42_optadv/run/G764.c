static int f1(int *p, int *q) { *p += 785; return *q * 2; }
static int f2(int *p, int *q) { *p += 713; return *q * 2; }
static int f3(int *p, int *q) { *p += 214; return *q * 2; }
static int f4(int *p, int *q) { *p += 387; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[2] + 563;
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    arr[1 % 6] = 313; cells[3] += arr[1 % 6];
    sum += f1(&cells[1], &cells[3]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f2(&cells[6], &cells[4]);
    cells[3] = cells[1] + 445;
    cells[3] = cells[1] + 170;
    cells[1] = cells[1] + 375;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 666u; }
    cells[2] = cells[5] + 393;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[3], &cells[0]);
    cells[0] = cells[1] + 219;
    sum += f4(&cells[3], &cells[0]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 695u; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }