static int f1(int *p, int *q) { *p += 825; return *q * 2; }
static int f2(int *p, int *q) { *p += 308; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 2u; }
    sum += f1(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[6] = cells[5] + 78;
    sum += f2(&cells[3], &cells[4]);
    cells[2] = cells[1] + 645;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 573u; }
    arr[5 % 6] = 97; cells[6] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 378u; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 613u; }
    cells[1] = cells[6] + 665;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 90u; }
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }