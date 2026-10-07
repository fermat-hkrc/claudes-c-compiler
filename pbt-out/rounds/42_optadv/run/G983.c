static int f1(int *p, int *q) { *p += 828; return *q * 2; }
static int f2(int *p, int *q) { *p += 245; return *q * 2; }
static int f3(int *p, int *q) { *p += 70; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[5] + 264;
    sum += f1(&cells[7], &cells[7]);
    arr[3 % 6] = 584; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 264u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 350u; }
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 744u; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[0 % 6] = 44; cells[3] += arr[0 % 6];
    arr[1 % 6] = 723; cells[4] += arr[1 % 6];
    cells[0] = cells[5] + 265;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 668u; }
    arr[1 % 6] = 209; cells[0] += arr[1 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 182u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 148u; }
    sum += f2(&cells[2], &cells[5]);
    cells[1] = cells[0] + 663;
    arr[5 % 6] = 457; cells[0] += arr[5 % 6];
    arr[2 % 6] = 421; cells[0] += arr[2 % 6];
    sum += f3(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }