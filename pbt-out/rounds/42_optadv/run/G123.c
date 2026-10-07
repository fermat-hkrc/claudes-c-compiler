static int f1(int *p, int *q) { *p += 353; return *q * 2; }
static int f2(int *p, int *q) { *p += 775; return *q * 2; }
static int f3(int *p, int *q) { *p += 749; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[1] + 311;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 460u; }
    arr[3 % 6] = 760; cells[5] += arr[3 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 244u; }
    cells[4] = cells[1] + 373;
    arr[2 % 6] = 566; cells[1] += arr[2 % 6];
    sum += f1(&cells[4], &cells[2]);
    sum += f2(&cells[7], &cells[4]);
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[0] + 74;
    arr[1 % 6] = 368; cells[6] += arr[1 % 6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f3(&cells[2], &cells[6]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 650u; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 743u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }