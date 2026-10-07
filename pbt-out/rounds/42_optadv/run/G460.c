static int f1(int *p, int *q) { *p += 952; return *q * 2; }
static int f2(int *p, int *q) { *p += 935; return *q * 2; }
static int f3(int *p, int *q) { *p += 460; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[3] + 134;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += f1(&cells[0], &cells[3]);
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 548u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[1] = cells[2] + 76;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f2(&cells[1], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 505u; }
    arr[3 % 6] = 838; cells[1] += arr[3 % 6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += f3(&cells[6], &cells[4]);
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 141u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[6] = cells[7] + 150;
    cells[6] = cells[5] + 703;
    cells[3] = cells[5] + 657;
    cells[4] = cells[6] + 64;
    arr[1 % 6] = 187; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }