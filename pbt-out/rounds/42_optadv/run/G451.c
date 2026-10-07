static int f1(int *p, int *q) { *p += 340; return *q * 2; }
static int f2(int *p, int *q) { *p += 224; return *q * 2; }
static int f3(int *p, int *q) { *p += 869; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[2], &cells[5]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 958u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 543u; }
    arr[1 % 6] = 837; cells[7] += arr[1 % 6];
    sum += f2(&cells[5], &cells[1]);
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[3] = cells[0] + 198;
    cells[3] = cells[2] + 836;
    cells[5] = cells[4] + 676;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[3] = cells[1] + 0;
    sum += f3(&cells[4], &cells[1]);
    cells[3] = cells[1] + 851;
    arr[3 % 6] = 580; cells[1] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }