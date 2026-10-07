static int f1(int *p, int *q) { *p += 137; return *q * 2; }
static int f2(int *p, int *q) { *p += 422; return *q * 2; }
static int f3(int *p, int *q) { *p += 53; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[6 % 6] = 91; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[6] + 942;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 628u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f1(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 318u; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    arr[5 % 6] = 925; cells[2] += arr[5 % 6];
    arr[2 % 6] = 746; cells[5] += arr[2 % 6];
    sum += f2(&cells[3], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += f3(&cells[5], &cells[1]);
    cells[6] = cells[4] + 210;
    arr[1 % 6] = 471; cells[2] += arr[1 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[2 % 6] = 966; cells[6] += arr[2 % 6];
    cells[4] = cells[4] + 54;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }