static int f1(int *p, int *q) { *p += 683; return *q * 2; }
static int f2(int *p, int *q) { *p += 727; return *q * 2; }
static int f3(int *p, int *q) { *p += 526; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 386u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 139u; }
    sum += f1(&cells[5], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[3] + 443;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    arr[6 % 6] = 389; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[4], &cells[7]);
    arr[4 % 6] = 744; cells[5] += arr[4 % 6];
    cells[6] = cells[7] + 954;
    arr[5 % 6] = 299; cells[3] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[7] = cells[2] + 222;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += f3(&cells[6], &cells[0]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 136u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }