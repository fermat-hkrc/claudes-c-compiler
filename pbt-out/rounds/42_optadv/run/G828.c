static int f1(int *p, int *q) { *p += 377; return *q * 2; }
static int f2(int *p, int *q) { *p += 118; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[4], &cells[5]);
    cells[2] = cells[1] + 662;
    cells[5] = cells[5] + 453;
    arr[7 % 6] = 790; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    arr[6 % 6] = 351; cells[5] += arr[6 % 6];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 800u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 349u; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[6] = cells[3] + 848;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    cells[5] = cells[0] + 976;
    cells[7] = cells[5] + 326;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 996u; }
    arr[7 % 6] = 618; cells[0] += arr[7 % 6];
    sum += f2(&cells[6], &cells[1]);
    cells[4] = cells[2] + 270;
    arr[0 % 6] = 273; cells[2] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }