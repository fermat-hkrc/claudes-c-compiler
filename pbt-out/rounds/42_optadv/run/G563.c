static int f1(int *p, int *q) { *p += 540; return *q * 2; }
static int f2(int *p, int *q) { *p += 29; return *q * 2; }
static int f3(int *p, int *q) { *p += 413; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 505; cells[6] += arr[7 % 6];
    cells[6] = cells[5] + 192;
    cells[7] = cells[4] + 195;
    sum += f1(&cells[1], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 471u; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    arr[6 % 6] = 525; cells[0] += arr[6 % 6];
    arr[1 % 6] = 83; cells[6] += arr[1 % 6];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    arr[3 % 6] = 125; cells[5] += arr[3 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    arr[2 % 6] = 284; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[1], &cells[4]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 59u; }
    cells[2] = cells[5] + 543;
    sum += f3(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }