static int f1(int *p, int *q) { *p += 643; return *q * 2; }
static int f2(int *p, int *q) { *p += 858; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 887u; }
    cells[5] = cells[5] + 177;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 568u; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[4], &cells[4]);
    arr[5 % 6] = 296; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 98; cells[7] += arr[4 % 6];
    sum += f2(&cells[2], &cells[0]);
    arr[3 % 6] = 13; cells[5] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }