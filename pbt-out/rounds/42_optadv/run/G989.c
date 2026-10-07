static int f1(int *p, int *q) { *p += 971; return *q * 2; }
static int f2(int *p, int *q) { *p += 880; return *q * 2; }
static int f3(int *p, int *q) { *p += 478; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 918u; }
    cells[0] = cells[4] + 533;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    arr[2 % 6] = 664; cells[7] += arr[2 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += f2(&cells[1], &cells[3]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    arr[3 % 6] = 259; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[5] = cells[5] + 553;
    sum += f3(&cells[0], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }