static int f1(int *p, int *q) { *p += 823; return *q * 2; }
static int f2(int *p, int *q) { *p += 551; return *q * 2; }
static int f3(int *p, int *q) { *p += 864; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[6] + 598;
    cells[0] = cells[4] + 924;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[3] = cells[5] + 579;
    sum += f1(&cells[0], &cells[1]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 350u; }
    sum += f2(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f3(&cells[7], &cells[0]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 787u; }
    arr[5 % 6] = 423; cells[3] += arr[5 % 6];
    arr[6 % 6] = 880; cells[4] += arr[6 % 6];
    cells[7] = cells[1] + 247;
    cells[1] = cells[1] + 843;
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }