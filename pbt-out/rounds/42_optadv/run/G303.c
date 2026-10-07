static int f1(int *p, int *q) { *p += 179; return *q * 2; }
static int f2(int *p, int *q) { *p += 800; return *q * 2; }
static int f3(int *p, int *q) { *p += 325; return *q * 2; }
static int f4(int *p, int *q) { *p += 601; return *q * 2; }
static int f5(int *p, int *q) { *p += 966; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 478; cells[0] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 575u; }
    sum += f1(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 73u; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 275u; }
    sum += f2(&cells[2], &cells[7]);
    sum += f3(&cells[4], &cells[2]);
    arr[0 % 6] = 314; cells[6] += arr[0 % 6];
    cells[7] = cells[0] + 107;
    sum += f4(&cells[0], &cells[0]);
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 465u; }
    arr[2 % 6] = 744; cells[1] += arr[2 % 6];
    sum += f5(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[6] = cells[3] + 777;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }