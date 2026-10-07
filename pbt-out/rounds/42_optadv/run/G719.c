static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 875; cells[1] += arr[1 % 6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    arr[1 % 6] = 985; cells[7] += arr[1 % 6];
    arr[2 % 6] = 942; cells[5] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[1 % 6] = 358; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[1] = cells[7] + 448;
    static int f1(int *p, int *q) { *p += 541; return *q * 2; }
    sum += f1(&cells[4], &cells[6]);
    cells[2] = cells[2] + 585;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 796u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }