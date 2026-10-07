static int f1(int *p, int *q) { *p += 557; return *q * 2; }
static int f2(int *p, int *q) { *p += 909; return *q * 2; }
static int f3(int *p, int *q) { *p += 912; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[0] + 289;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[7 % 6] = 867; cells[2] += arr[7 % 6];
    cells[5] = cells[3] + 433;
    sum += f1(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[2 % 6] = 391; cells[2] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += f2(&cells[0], &cells[6]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 685u; }
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    sum += f3(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }