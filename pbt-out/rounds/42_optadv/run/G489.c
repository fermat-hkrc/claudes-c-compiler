static int f1(int *p, int *q) { *p += 703; return *q * 2; }
static int f2(int *p, int *q) { *p += 285; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 197; cells[3] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[7 % 6] = 266; cells[6] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 158u; }
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 83u; }
    sum += f1(&cells[7], &cells[4]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[5] = cells[3] + 369;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[5] = cells[7] + 691;
    sum += f2(&cells[4], &cells[7]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 845u; }
    arr[6 % 6] = 673; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 156u; }
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }