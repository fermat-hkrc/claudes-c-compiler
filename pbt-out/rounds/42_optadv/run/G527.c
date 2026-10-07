static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 744; cells[3] += arr[2 % 6];
    arr[4 % 6] = 780; cells[3] += arr[4 % 6];
    cells[5] = cells[0] + 698;
    cells[1] = cells[1] + 863;
    static int f1(int *p, int *q) { *p += 850; return *q * 2; }
    sum += f1(&cells[7], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 644; cells[1] += arr[5 % 6];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 493u; }
    cells[6] = cells[3] + 646;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    static int f2(int *p, int *q) { *p += 217; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[4] = cells[0] + 241;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[1] = cells[0] + 744;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 724u; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f3(int *p, int *q) { *p += 154; return *q * 2; }
    sum += f3(&cells[2], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }