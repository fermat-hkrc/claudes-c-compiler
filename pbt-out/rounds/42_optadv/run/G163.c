static int f1(int *p, int *q) { *p += 921; return *q * 2; }
static int f2(int *p, int *q) { *p += 251; return *q * 2; }
static int f3(int *p, int *q) { *p += 316; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 921u; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 194u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 461u; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 328u; }
    cells[0] = cells[3] + 664;
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[4 % 6] = 621; cells[6] += arr[4 % 6];
    cells[5] = cells[1] + 113;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += f1(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[2 % 6] = 530; cells[0] += arr[2 % 6];
    sum += f2(&cells[2], &cells[0]);
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += f3(&cells[7], &cells[4]);
    cells[3] = cells[5] + 658;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 283u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }