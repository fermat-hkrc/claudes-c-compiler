static int f1(int *p, int *q) { *p += 969; return *q * 2; }
static int f2(int *p, int *q) { *p += 689; return *q * 2; }
static int f3(int *p, int *q) { *p += 932; return *q * 2; }
static int f4(int *p, int *q) { *p += 276; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[3]);
    sum += f2(&cells[0], &cells[3]);
    sum += f3(&cells[1], &cells[7]);
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 917u; }
    arr[4 % 6] = 447; cells[0] += arr[4 % 6];
    sum += f4(&cells[2], &cells[1]);
    arr[1 % 6] = 287; cells[0] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[3] = cells[0] + 718;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[0] = cells[3] + 26;
    cells[6] = cells[1] + 434;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }