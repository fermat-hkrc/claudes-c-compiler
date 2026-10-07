static int f1(int *p, int *q) { *p += 471; return *q * 2; }
static int f2(int *p, int *q) { *p += 940; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[3 % 6] = 252; cells[6] += arr[3 % 6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    arr[5 % 6] = 573; cells[4] += arr[5 % 6];
    cells[3] = cells[0] + 834;
    arr[7 % 6] = 793; cells[0] += arr[7 % 6];
    cells[6] = cells[3] + 357;
    arr[7 % 6] = 252; cells[3] += arr[7 % 6];
    sum += f1(&cells[5], &cells[0]);
    cells[5] = cells[5] + 157;
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 829u; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 614; cells[6] += arr[3 % 6];
    sum += f2(&cells[3], &cells[6]);
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    cells[4] = cells[1] + 448;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 308u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }