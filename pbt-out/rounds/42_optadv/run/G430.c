static int f1(int *p, int *q) { *p += 21; return *q * 2; }
static int f2(int *p, int *q) { *p += 362; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[2] = cells[3] + 639;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 868u; }
    sum += f2(&cells[6], &cells[6]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[6 % 6] = 951; cells[3] += arr[6 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[7 % 6] = 353; cells[1] += arr[7 % 6];
    cells[2] = cells[7] + 855;
    cells[5] = cells[2] + 221;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }