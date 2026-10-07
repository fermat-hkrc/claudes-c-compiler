static int f1(int *p, int *q) { *p += 648; return *q * 2; }
static int f2(int *p, int *q) { *p += 909; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 477u; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += f1(&cells[4], &cells[6]);
    sum += f2(&cells[3], &cells[7]);
    arr[5 % 6] = 325; cells[5] += arr[5 % 6];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 230u; }
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[1] = cells[7] + 102;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[6] = cells[1] + 688;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[7 % 6] = 968; cells[6] += arr[7 % 6];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    cells[6] = cells[5] + 773;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 549u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }