static int f1(int *p, int *q) { *p += 741; return *q * 2; }
static int f2(int *p, int *q) { *p += 615; return *q * 2; }
static int f3(int *p, int *q) { *p += 669; return *q * 2; }
static int f4(int *p, int *q) { *p += 165; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 556; cells[4] += arr[3 % 6];
    sum += f1(&cells[2], &cells[7]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 748u; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 1u; }
    sum += f2(&cells[3], &cells[3]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[4 % 6] = 403; cells[4] += arr[4 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[0 % 6] = 789; cells[1] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 151u; }
    cells[5] = cells[1] + 143;
    cells[3] = cells[0] + 675;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 529u; }
    cells[1] = cells[2] + 631;
    cells[6] = cells[7] + 179;
    sum += f3(&cells[0], &cells[4]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f4(&cells[2], &cells[4]);
    arr[2 % 6] = 744; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }