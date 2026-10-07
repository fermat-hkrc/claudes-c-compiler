static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[3] + 698;
    cells[7] = cells[6] + 566;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 554u; }
    arr[1 % 6] = 963; cells[7] += arr[1 % 6];
    static int f1(int *p, int *q) { *p += 360; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f2(int *p, int *q) { *p += 644; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 684; return *q * 2; }
    sum += f3(&cells[0], &cells[6]);
    arr[5 % 6] = 61; cells[7] += arr[5 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[5 % 6] = 44; cells[7] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }