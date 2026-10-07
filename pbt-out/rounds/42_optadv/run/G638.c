static int f1(int *p, int *q) { *p += 591; return *q * 2; }
static int f2(int *p, int *q) { *p += 707; return *q * 2; }
static int f3(int *p, int *q) { *p += 591; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[5]);
    arr[5 % 6] = 659; cells[4] += arr[5 % 6];
    cells[4] = cells[3] + 309;
    cells[4] = cells[1] + 757;
    arr[6 % 6] = 873; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 603u; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += f2(&cells[4], &cells[5]);
    sum += f3(&cells[4], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }