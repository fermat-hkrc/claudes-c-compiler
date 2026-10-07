static int f1(int *p, int *q) { *p += 506; return *q * 2; }
static int f2(int *p, int *q) { *p += 798; return *q * 2; }
static int f3(int *p, int *q) { *p += 619; return *q * 2; }
static int f4(int *p, int *q) { *p += 374; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += f1(&cells[0], &cells[2]);
    cells[7] = cells[2] + 666;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += f2(&cells[5], &cells[4]);
    sum += f3(&cells[3], &cells[2]);
    arr[1 % 6] = 314; cells[6] += arr[1 % 6];
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    arr[5 % 6] = 867; cells[2] += arr[5 % 6];
    cells[7] = cells[7] + 798;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 704u; }
    arr[1 % 6] = 353; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[0], &cells[5]);
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 932u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 768u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 463u; }
    arr[2 % 6] = 813; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }