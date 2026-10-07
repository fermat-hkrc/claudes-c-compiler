static int f1(int *p, int *q) { *p += 938; return *q * 2; }
static int f2(int *p, int *q) { *p += 988; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 756u; }
    arr[6 % 6] = 107; cells[2] += arr[6 % 6];
    arr[0 % 6] = 933; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    arr[4 % 6] = 575; cells[1] += arr[4 % 6];
    cells[0] = cells[0] + 441;
    sum += f1(&cells[0], &cells[7]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[5 % 6] = 980; cells[5] += arr[5 % 6];
    arr[0 % 6] = 992; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    arr[6 % 6] = 886; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 496; cells[3] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[0], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[2] + 57;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[2 % 6] = 767; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }