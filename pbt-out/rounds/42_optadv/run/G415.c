static int f1(int *p, int *q) { *p += 269; return *q * 2; }
static int f2(int *p, int *q) { *p += 536; return *q * 2; }
static int f3(int *p, int *q) { *p += 859; return *q * 2; }
static int f4(int *p, int *q) { *p += 898; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 180; cells[7] += arr[7 % 6];
    sum += f1(&cells[4], &cells[0]);
    cells[7] = cells[2] + 826;
    arr[4 % 6] = 353; cells[0] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 379u; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 593; cells[5] += arr[5 % 6];
    cells[7] = cells[0] + 195;
    cells[7] = cells[7] + 277;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[1] + 666;
    sum += f2(&cells[3], &cells[2]);
    sum += f3(&cells[4], &cells[1]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 875u; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    arr[0 % 6] = 66; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += f4(&cells[0], &cells[3]);
    arr[1 % 6] = 290; cells[6] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }