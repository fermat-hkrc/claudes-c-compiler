static int f1(int *p, int *q) { *p += 222; return *q * 2; }
static int f2(int *p, int *q) { *p += 977; return *q * 2; }
static int f3(int *p, int *q) { *p += 202; return *q * 2; }
static int f4(int *p, int *q) { *p += 621; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[4] = cells[0] + 913;
    sum += f1(&cells[3], &cells[4]);
    sum += f2(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f3(&cells[4], &cells[0]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 75u; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    cells[0] = cells[7] + 503;
    arr[1 % 6] = 143; cells[4] += arr[1 % 6];
    cells[6] = cells[2] + 769;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += f4(&cells[5], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[6] + 249;
    arr[3 % 6] = 467; cells[1] += arr[3 % 6];
    cells[7] = cells[5] + 42;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }