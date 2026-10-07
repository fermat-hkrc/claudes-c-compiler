static int f1(int *p, int *q) { *p += 834; return *q * 2; }
static int f2(int *p, int *q) { *p += 278; return *q * 2; }
static int f3(int *p, int *q) { *p += 9; return *q * 2; }
static int f4(int *p, int *q) { *p += 613; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 584u; }
    arr[2 % 6] = 890; cells[5] += arr[2 % 6];
    arr[3 % 6] = 744; cells[4] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[7] = cells[2] + 951;
    cells[3] = cells[5] + 838;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 682; cells[5] += arr[0 % 6];
    arr[5 % 6] = 101; cells[6] += arr[5 % 6];
    arr[7 % 6] = 255; cells[6] += arr[7 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    arr[6 % 6] = 926; cells[3] += arr[6 % 6];
    sum += f1(&cells[4], &cells[7]);
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 46u; }
    arr[4 % 6] = 721; cells[7] += arr[4 % 6];
    sum += f2(&cells[7], &cells[0]);
    arr[6 % 6] = 760; cells[5] += arr[6 % 6];
    sum += f3(&cells[5], &cells[6]);
    cells[4] = cells[5] + 900;
    arr[7 % 6] = 318; cells[1] += arr[7 % 6];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 388u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f4(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }