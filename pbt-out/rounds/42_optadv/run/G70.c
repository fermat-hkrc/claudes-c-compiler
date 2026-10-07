static int f1(int *p, int *q) { *p += 369; return *q * 2; }
static int f2(int *p, int *q) { *p += 848; return *q * 2; }
static int f3(int *p, int *q) { *p += 77; return *q * 2; }
static int f4(int *p, int *q) { *p += 823; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 741u; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 278u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += f1(&cells[4], &cells[2]);
    arr[0 % 6] = 175; cells[0] += arr[0 % 6];
    arr[1 % 6] = 827; cells[3] += arr[1 % 6];
    cells[4] = cells[5] + 479;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[6], &cells[3]);
    sum += f3(&cells[0], &cells[2]);
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 319; cells[6] += arr[2 % 6];
    sum += f4(&cells[7], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 34u; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[1] = cells[2] + 667;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }