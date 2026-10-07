static int f1(int *p, int *q) { *p += 61; return *q * 2; }
static int f2(int *p, int *q) { *p += 132; return *q * 2; }
static int f3(int *p, int *q) { *p += 897; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f1(&cells[4], &cells[3]);
    cells[0] = cells[2] + 580;
    arr[7 % 6] = 249; cells[0] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 659u; }
    cells[7] = cells[7] + 847;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += f2(&cells[3], &cells[0]);
    sum += f3(&cells[3], &cells[1]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    arr[4 % 6] = 426; cells[5] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }