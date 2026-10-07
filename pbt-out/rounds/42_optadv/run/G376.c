static int f1(int *p, int *q) { *p += 469; return *q * 2; }
static int f2(int *p, int *q) { *p += 319; return *q * 2; }
static int f3(int *p, int *q) { *p += 301; return *q * 2; }
static int f4(int *p, int *q) { *p += 635; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 194u; }
    sum += f1(&cells[2], &cells[1]);
    sum += f2(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f3(&cells[1], &cells[2]);
    cells[0] = cells[5] + 589;
    cells[2] = cells[4] + 142;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    cells[0] = cells[6] + 740;
    sum += f4(&cells[2], &cells[4]);
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 150u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[2 % 6] = 152; cells[4] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 481u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }