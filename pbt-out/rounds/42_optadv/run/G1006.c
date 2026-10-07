static int f1(int *p, int *q) { *p += 163; return *q * 2; }
static int f2(int *p, int *q) { *p += 747; return *q * 2; }
static int f3(int *p, int *q) { *p += 287; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 108u; }
    arr[0 % 6] = 311; cells[2] += arr[0 % 6];
    sum += f1(&cells[3], &cells[4]);
    sum += f2(&cells[3], &cells[3]);
    arr[3 % 6] = 365; cells[4] += arr[3 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 190u; }
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[4] + 252;
    cells[6] = cells[0] + 780;
    arr[6 % 6] = 540; cells[7] += arr[6 % 6];
    sum += f3(&cells[4], &cells[1]);
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 54u; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 535; cells[7] += arr[2 % 6];
    arr[4 % 6] = 119; cells[1] += arr[4 % 6];
    cells[1] = cells[6] + 515;
    arr[7 % 6] = 926; cells[2] += arr[7 % 6];
    arr[2 % 6] = 337; cells[3] += arr[2 % 6];
    arr[7 % 6] = 222; cells[3] += arr[7 % 6];
    arr[2 % 6] = 157; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }