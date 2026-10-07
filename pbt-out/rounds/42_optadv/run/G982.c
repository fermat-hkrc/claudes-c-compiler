static int f1(int *p, int *q) { *p += 474; return *q * 2; }
static int f2(int *p, int *q) { *p += 758; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 634; cells[2] += arr[4 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 556u; }
    arr[5 % 6] = 194; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[2 % 6] = 330; cells[5] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[1], &cells[2]);
    cells[7] = cells[2] + 500;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 503u; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += f2(&cells[6], &cells[7]);
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 732u; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }