static int f1(int *p, int *q) { *p += 258; return *q * 2; }
static int f2(int *p, int *q) { *p += 856; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[7]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[6] = cells[5] + 688;
    cells[0] = cells[6] + 650;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 360u; }
    arr[3 % 6] = 153; cells[0] += arr[3 % 6];
    sum += f2(&cells[4], &cells[2]);
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[0] = cells[2] + 212;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 395u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }