static int f1(int *p, int *q) { *p += 821; return *q * 2; }
static int f2(int *p, int *q) { *p += 684; return *q * 2; }
static int f3(int *p, int *q) { *p += 408; return *q * 2; }
static int f4(int *p, int *q) { *p += 70; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[4]);
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[2 % 6] = 772; cells[5] += arr[2 % 6];
    cells[6] = cells[6] + 269;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[1], &cells[1]);
    arr[7 % 6] = 589; cells[6] += arr[7 % 6];
    arr[6 % 6] = 891; cells[1] += arr[6 % 6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 348u; }
    arr[7 % 6] = 764; cells[3] += arr[7 % 6];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += f3(&cells[0], &cells[1]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += f4(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }