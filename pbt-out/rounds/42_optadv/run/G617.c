static int f1(int *p, int *q) { *p += 873; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 557u; }
    sum += f1(&cells[6], &cells[6]);
    arr[5 % 6] = 8; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    cells[6] = cells[5] + 234;
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    arr[7 % 6] = 545; cells[5] += arr[7 % 6];
    arr[0 % 6] = 861; cells[2] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }