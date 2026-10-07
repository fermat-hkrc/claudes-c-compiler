static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 495; return *q * 2; }
    sum += f1(&cells[3], &cells[2]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 635u; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[7 % 6] = 984; cells[3] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[4] = cells[1] + 10;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 469u; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    arr[6 % 6] = 947; cells[6] += arr[6 % 6];
    cells[6] = cells[6] + 554;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    static int f2(int *p, int *q) { *p += 59; return *q * 2; }
    sum += f2(&cells[5], &cells[0]);
    arr[2 % 6] = 571; cells[4] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }