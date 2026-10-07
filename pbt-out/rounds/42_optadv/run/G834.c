static int f1(int *p, int *q) { *p += 649; return *q * 2; }
static int f2(int *p, int *q) { *p += 768; return *q * 2; }
static int f3(int *p, int *q) { *p += 125; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[2]);
    sum += f2(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 534u; }
    cells[6] = cells[5] + 546;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 998u; }
    cells[1] = cells[7] + 362;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    cells[0] = cells[4] + 794;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 197u; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 181u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[4 % 6] = 183; cells[2] += arr[4 % 6];
    sum += f3(&cells[3], &cells[2]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }