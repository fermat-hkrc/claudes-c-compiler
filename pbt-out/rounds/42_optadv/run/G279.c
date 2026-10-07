static int f1(int *p, int *q) { *p += 811; return *q * 2; }
static int f2(int *p, int *q) { *p += 725; return *q * 2; }
static int f3(int *p, int *q) { *p += 196; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[1], &cells[5]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += f2(&cells[2], &cells[7]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 106u; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 758u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 131u; }
    sum += f3(&cells[7], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[4] + 220;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 130u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }