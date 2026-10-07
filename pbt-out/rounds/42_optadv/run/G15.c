static int f1(int *p, int *q) { *p += 201; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[2] + 652;
    sum += f1(&cells[0], &cells[1]);
    cells[4] = cells[5] + 662;
    cells[7] = cells[1] + 81;
    arr[0 % 6] = 233; cells[6] += arr[0 % 6];
    arr[0 % 6] = 362; cells[5] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 461u; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    cells[5] = cells[7] + 592;
    cells[6] = cells[6] + 964;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 315u; }
    arr[0 % 6] = 84; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[0] = cells[6] + 924;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }