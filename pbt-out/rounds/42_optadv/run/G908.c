static int f1(int *p, int *q) { *p += 626; return *q * 2; }
static int f2(int *p, int *q) { *p += 881; return *q * 2; }
static int f3(int *p, int *q) { *p += 563; return *q * 2; }
static int f4(int *p, int *q) { *p += 529; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 384u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 897u; }
    cells[6] = cells[3] + 145;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 549u; }
    arr[5 % 6] = 961; cells[3] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 440u; }
    cells[7] = cells[3] + 965;
    sum += f1(&cells[5], &cells[6]);
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    arr[5 % 6] = 17; cells[5] += arr[5 % 6];
    cells[7] = cells[1] + 322;
    sum += f3(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    arr[2 % 6] = 270; cells[1] += arr[2 % 6];
    sum += f4(&cells[1], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }