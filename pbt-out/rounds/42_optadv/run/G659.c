static int f1(int *p, int *q) { *p += 336; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[1], &cells[3]);
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[5 % 6] = 846; cells[1] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 294u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[4 % 6] = 650; cells[7] += arr[4 % 6];
    arr[7 % 6] = 900; cells[5] += arr[7 % 6];
    arr[5 % 6] = 589; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 524u; }
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    cells[5] = cells[4] + 568;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    cells[1] = cells[3] + 952;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    arr[1 % 6] = 353; cells[7] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }