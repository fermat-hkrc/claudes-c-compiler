static int f1(int *p, int *q) { *p += 239; return *q * 2; }
static int f2(int *p, int *q) { *p += 567; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    cells[0] = cells[1] + 856;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[1 % 6] = 250; cells[5] += arr[1 % 6];
    sum += f1(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f2(&cells[5], &cells[5]);
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 231u; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 752u; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 41u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }