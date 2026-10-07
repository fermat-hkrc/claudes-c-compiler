static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 789; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[3 % 6] = 113; cells[2] += arr[3 % 6];
    cells[1] = cells[4] + 736;
    static int f2(int *p, int *q) { *p += 250; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 16u; }
    cells[6] = cells[5] + 621;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    arr[0 % 6] = 94; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f3(int *p, int *q) { *p += 623; return *q * 2; }
    sum += f3(&cells[3], &cells[3]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[2 % 6] = 711; cells[6] += arr[2 % 6];
    cells[4] = cells[0] + 298;
    arr[2 % 6] = 967; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[1 % 6] = 6; cells[3] += arr[1 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[3 % 6] = 946; cells[7] += arr[3 % 6];
    arr[6 % 6] = 838; cells[5] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 241u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }