static int f1(int *p, int *q) { *p += 444; return *q * 2; }
static int f2(int *p, int *q) { *p += 391; return *q * 2; }
static int f3(int *p, int *q) { *p += 643; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 354u; }
    arr[6 % 6] = 377; cells[2] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[7] = cells[7] + 275;
    arr[3 % 6] = 163; cells[1] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 408u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    cells[4] = cells[4] + 776;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[1 % 6] = 407; cells[4] += arr[1 % 6];
    cells[0] = cells[0] + 352;
    sum += f1(&cells[3], &cells[0]);
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 282u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 524u; }
    sum += f2(&cells[6], &cells[4]);
    cells[6] = cells[5] + 523;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f3(&cells[7], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }