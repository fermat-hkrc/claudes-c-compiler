static int f1(int *p, int *q) { *p += 350; return *q * 2; }
static int f2(int *p, int *q) { *p += 71; return *q * 2; }
static int f3(int *p, int *q) { *p += 87; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[1]);
    arr[7 % 6] = 340; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[3] = cells[6] + 462;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 497u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[2] = cells[3] + 754;
    sum += f2(&cells[3], &cells[2]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 154u; }
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    cells[7] = cells[5] + 388;
    arr[6 % 6] = 148; cells[6] += arr[6 % 6];
    sum += f3(&cells[1], &cells[6]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }