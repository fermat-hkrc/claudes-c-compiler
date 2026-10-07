static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[5] + 615;
    cells[0] = cells[0] + 379;
    cells[1] = cells[4] + 12;
    static int f1(int *p, int *q) { *p += 754; return *q * 2; }
    sum += f1(&cells[4], &cells[3]);
    arr[0 % 6] = 445; cells[7] += arr[0 % 6];
    cells[5] = cells[6] + 977;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[1 % 6] = 176; cells[3] += arr[1 % 6];
    cells[7] = cells[6] + 286;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 549u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    static int f2(int *p, int *q) { *p += 125; return *q * 2; }
    sum += f2(&cells[3], &cells[7]);
    cells[1] = cells[6] + 393;
    arr[5 % 6] = 303; cells[4] += arr[5 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }