static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[4] + 521;
    static int f1(int *p, int *q) { *p += 558; return *q * 2; }
    sum += f1(&cells[4], &cells[2]);
    arr[5 % 6] = 870; cells[6] += arr[5 % 6];
    static int f2(int *p, int *q) { *p += 801; return *q * 2; }
    sum += f2(&cells[5], &cells[4]);
    arr[6 % 6] = 826; cells[2] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 221u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[5] = cells[5] + 288;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[2 % 6] = 364; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 953; return *q * 2; }
    sum += f3(&cells[6], &cells[2]);
    cells[4] = cells[3] + 372;
    arr[0 % 6] = 50; cells[3] += arr[0 % 6];
    arr[3 % 6] = 582; cells[0] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }