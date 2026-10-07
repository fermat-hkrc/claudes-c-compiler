static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 942; cells[7] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 587; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 60u; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    arr[3 % 6] = 463; cells[6] += arr[3 % 6];
    static int f2(int *p, int *q) { *p += 627; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    static int f3(int *p, int *q) { *p += 575; return *q * 2; }
    sum += f3(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[1] = cells[4] + 870;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 528u; }
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[7 % 6] = 383; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    arr[2 % 6] = 476; cells[5] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    static int f4(int *p, int *q) { *p += 653; return *q * 2; }
    sum += f4(&cells[3], &cells[3]);
    cells[2] = cells[7] + 656;
    cells[4] = cells[7] + 372;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }