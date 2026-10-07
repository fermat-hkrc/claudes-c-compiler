static int f1(int *p, int *q) { *p += 981; return *q * 2; }
static int f2(int *p, int *q) { *p += 571; return *q * 2; }
static int f3(int *p, int *q) { *p += 757; return *q * 2; }
static int f4(int *p, int *q) { *p += 885; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 458u; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    cells[7] = cells[7] + 710;
    cells[7] = cells[3] + 351;
    cells[7] = cells[0] + 199;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[3], &cells[5]);
    sum += f2(&cells[2], &cells[7]);
    cells[6] = cells[7] + 123;
    sum += f3(&cells[3], &cells[6]);
    arr[0 % 6] = 205; cells[7] += arr[0 % 6];
    cells[1] = cells[4] + 116;
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 447u; }
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 352u; }
    sum += f4(&cells[5], &cells[3]);
    arr[7 % 6] = 593; cells[6] += arr[7 % 6];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }