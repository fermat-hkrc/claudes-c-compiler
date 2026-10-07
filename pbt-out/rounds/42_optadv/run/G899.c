static int f1(int *p, int *q) { *p += 985; return *q * 2; }
static int f2(int *p, int *q) { *p += 615; return *q * 2; }
static int f3(int *p, int *q) { *p += 621; return *q * 2; }
static int f4(int *p, int *q) { *p += 611; return *q * 2; }
static int f5(int *p, int *q) { *p += 616; return *q * 2; }
static int f6(int *p, int *q) { *p += 665; return *q * 2; }
static int f7(int *p, int *q) { *p += 348; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 331; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[1] + 912;
    cells[5] = cells[7] + 120;
    sum += f2(&cells[2], &cells[4]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 901u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[5], &cells[0]);
    sum += f4(&cells[4], &cells[4]);
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 113u; }
    cells[1] = cells[5] + 60;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 465u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 382u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f5(&cells[7], &cells[1]);
    sum += f6(&cells[1], &cells[6]);
    arr[2 % 6] = 298; cells[2] += arr[2 % 6];
    sum += f7(&cells[6], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }