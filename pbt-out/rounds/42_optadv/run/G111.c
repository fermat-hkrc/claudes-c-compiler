static int f1(int *p, int *q) { *p += 674; return *q * 2; }
static int f2(int *p, int *q) { *p += 543; return *q * 2; }
static int f3(int *p, int *q) { *p += 433; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[4], &cells[7]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 418u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 886; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[2], &cells[2]);
    cells[3] = cells[7] + 751;
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    arr[2 % 6] = 133; cells[0] += arr[2 % 6];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += f3(&cells[6], &cells[5]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[4 % 6] = 850; cells[4] += arr[4 % 6];
    arr[5 % 6] = 735; cells[3] += arr[5 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }