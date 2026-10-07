static int f1(int *p, int *q) { *p += 223; return *q * 2; }
static int f2(int *p, int *q) { *p += 257; return *q * 2; }
static int f3(int *p, int *q) { *p += 650; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 628; cells[0] += arr[6 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[2 % 6] = 952; cells[4] += arr[2 % 6];
    cells[0] = cells[5] + 817;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 658u; }
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 860u; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 934; cells[4] += arr[2 % 6];
    sum += f1(&cells[6], &cells[1]);
    cells[4] = cells[5] + 947;
    sum += f2(&cells[5], &cells[5]);
    cells[2] = cells[3] + 730;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f3(&cells[4], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }