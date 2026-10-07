static int f1(int *p, int *q) { *p += 696; return *q * 2; }
static int f2(int *p, int *q) { *p += 314; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[3]);
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 414u; }
    cells[4] = cells[0] + 522;
    cells[3] = cells[0] + 865;
    arr[1 % 6] = 660; cells[7] += arr[1 % 6];
    arr[5 % 6] = 581; cells[1] += arr[5 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 14u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 604u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[2], &cells[3]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[5] = cells[0] + 577;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 726u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }