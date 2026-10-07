static int f1(int *p, int *q) { *p += 510; return *q * 2; }
static int f2(int *p, int *q) { *p += 53; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 990u; }
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    cells[7] = cells[4] + 917;
    cells[6] = cells[2] + 933;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += f1(&cells[5], &cells[1]);
    arr[2 % 6] = 958; cells[7] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    cells[4] = cells[6] + 350;
    sum += f2(&cells[7], &cells[7]);
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }