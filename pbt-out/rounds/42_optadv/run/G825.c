static int f1(int *p, int *q) { *p += 908; return *q * 2; }
static int f2(int *p, int *q) { *p += 884; return *q * 2; }
static int f3(int *p, int *q) { *p += 433; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += f1(&cells[4], &cells[4]);
    arr[1 % 6] = 706; cells[2] += arr[1 % 6];
    arr[2 % 6] = 292; cells[1] += arr[2 % 6];
    cells[5] = cells[4] + 512;
    sum += f2(&cells[4], &cells[4]);
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[0 % 6] = 30; cells[2] += arr[0 % 6];
    arr[1 % 6] = 250; cells[7] += arr[1 % 6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 466u; }
    sum += f3(&cells[2], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }