static int f1(int *p, int *q) { *p += 78; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[0] + 780;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[0] = cells[6] + 497;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 920; cells[2] += arr[3 % 6];
    sum += f1(&cells[7], &cells[7]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 409u; }
    cells[4] = cells[2] + 451;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    cells[7] = cells[0] + 186;
    cells[5] = cells[3] + 256;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 785; cells[4] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }