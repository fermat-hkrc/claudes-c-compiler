static int f1(int *p, int *q) { *p += 868; return *q * 2; }
static int f2(int *p, int *q) { *p += 226; return *q * 2; }
static int f3(int *p, int *q) { *p += 235; return *q * 2; }
static int f4(int *p, int *q) { *p += 885; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[6] + 391;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[0 % 6] = 235; cells[3] += arr[0 % 6];
    sum += f1(&cells[0], &cells[5]);
    sum += f2(&cells[6], &cells[5]);
    sum += f3(&cells[2], &cells[7]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 973u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    sum += f4(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 245u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }