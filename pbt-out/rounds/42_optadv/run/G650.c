static int f1(int *p, int *q) { *p += 682; return *q * 2; }
static int f2(int *p, int *q) { *p += 524; return *q * 2; }
static int f3(int *p, int *q) { *p += 721; return *q * 2; }
static int f4(int *p, int *q) { *p += 40; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f1(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 118u; }
    sum += f2(&cells[2], &cells[3]);
    cells[3] = cells[7] + 13;
    sum += f3(&cells[2], &cells[4]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[4 % 6] = 631; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += f4(&cells[6], &cells[3]);
    cells[6] = cells[4] + 41;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }