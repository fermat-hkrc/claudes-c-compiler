static int f1(int *p, int *q) { *p += 760; return *q * 2; }
static int f2(int *p, int *q) { *p += 27; return *q * 2; }
static int f3(int *p, int *q) { *p += 883; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[5]);
    sum += f2(&cells[5], &cells[3]);
    sum += f3(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    cells[4] = cells[3] + 622;
    cells[4] = cells[4] + 532;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    cells[7] = cells[4] + 638;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 152u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[2 % 6] = 142; cells[2] += arr[2 % 6];
    arr[1 % 6] = 72; cells[6] += arr[1 % 6];
    arr[3 % 6] = 430; cells[4] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }