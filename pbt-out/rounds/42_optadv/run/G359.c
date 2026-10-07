static int f1(int *p, int *q) { *p += 951; return *q * 2; }
static int f2(int *p, int *q) { *p += 690; return *q * 2; }
static int f3(int *p, int *q) { *p += 618; return *q * 2; }
static int f4(int *p, int *q) { *p += 64; return *q * 2; }
static int f5(int *p, int *q) { *p += 575; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[2]);
    arr[4 % 6] = 979; cells[0] += arr[4 % 6];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[1] = cells[7] + 612;
    cells[2] = cells[0] + 888;
    arr[0 % 6] = 928; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[3] + 652;
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 347u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[2] = cells[3] + 252;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[2], &cells[1]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += f3(&cells[3], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[5], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 686u; }
    sum += f5(&cells[5], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }