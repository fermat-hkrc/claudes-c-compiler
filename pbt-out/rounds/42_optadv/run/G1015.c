static int f1(int *p, int *q) { *p += 490; return *q * 2; }
static int f2(int *p, int *q) { *p += 913; return *q * 2; }
static int f3(int *p, int *q) { *p += 412; return *q * 2; }
static int f4(int *p, int *q) { *p += 513; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[1] = cells[0] + 864;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 934u; }
    sum += f1(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f2(&cells[5], &cells[2]);
    sum += f3(&cells[2], &cells[4]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 188u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 550u; }
    sum += f4(&cells[4], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    arr[3 % 6] = 234; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 18; cells[3] += arr[5 % 6];
    cells[6] = cells[3] + 207;
    arr[6 % 6] = 220; cells[4] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }