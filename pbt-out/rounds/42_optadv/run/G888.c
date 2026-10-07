static int f1(int *p, int *q) { *p += 735; return *q * 2; }
static int f2(int *p, int *q) { *p += 968; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 15u; }
    arr[2 % 6] = 785; cells[7] += arr[2 % 6];
    sum += f1(&cells[0], &cells[2]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 607u; }
    arr[6 % 6] = 251; cells[2] += arr[6 % 6];
    arr[1 % 6] = 351; cells[6] += arr[1 % 6];
    cells[7] = cells[0] + 38;
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    arr[7 % 6] = 10; cells[2] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[6 % 6] = 398; cells[2] += arr[6 % 6];
    sum += f2(&cells[7], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }