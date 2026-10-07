static int f1(int *p, int *q) { *p += 182; return *q * 2; }
static int f2(int *p, int *q) { *p += 497; return *q * 2; }
static int f3(int *p, int *q) { *p += 456; return *q * 2; }
static int f4(int *p, int *q) { *p += 105; return *q * 2; }
static int f5(int *p, int *q) { *p += 497; return *q * 2; }
static int f6(int *p, int *q) { *p += 843; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[7]);
    arr[4 % 6] = 165; cells[3] += arr[4 % 6];
    cells[3] = cells[7] + 558;
    cells[5] = cells[5] + 383;
    cells[3] = cells[6] + 155;
    cells[3] = cells[0] + 947;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += f2(&cells[2], &cells[3]);
    cells[5] = cells[5] + 489;
    cells[0] = cells[5] + 314;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 293u; }
    arr[1 % 6] = 166; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[1] = cells[6] + 596;
    arr[0 % 6] = 252; cells[5] += arr[0 % 6];
    arr[7 % 6] = 649; cells[1] += arr[7 % 6];
    sum += f3(&cells[5], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f4(&cells[4], &cells[6]);
    sum += f5(&cells[5], &cells[5]);
    sum += f6(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }