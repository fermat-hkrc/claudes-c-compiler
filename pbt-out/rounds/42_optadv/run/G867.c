static int f1(int *p, int *q) { *p += 114; return *q * 2; }
static int f2(int *p, int *q) { *p += 306; return *q * 2; }
static int f3(int *p, int *q) { *p += 969; return *q * 2; }
static int f4(int *p, int *q) { *p += 195; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 395; cells[1] += arr[5 % 6];
    sum += f1(&cells[3], &cells[0]);
    arr[0 % 6] = 60; cells[7] += arr[0 % 6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    arr[4 % 6] = 99; cells[4] += arr[4 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f2(&cells[3], &cells[0]);
    arr[3 % 6] = 20; cells[0] += arr[3 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 281u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[1 % 6] = 482; cells[5] += arr[1 % 6];
    arr[4 % 6] = 861; cells[5] += arr[4 % 6];
    sum += f3(&cells[1], &cells[5]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 249u; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    sum += f4(&cells[7], &cells[1]);
    cells[5] = cells[7] + 300;
    cells[7] = cells[3] + 759;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }