static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[7] + 881;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 639u; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 120u; }
    cells[6] = cells[5] + 480;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    cells[6] = cells[4] + 225;
    static int f1(int *p, int *q) { *p += 412; return *q * 2; }
    sum += f1(&cells[4], &cells[3]);
    static int f2(int *p, int *q) { *p += 275; return *q * 2; }
    sum += f2(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 149u; }
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[0 % 6] = 602; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }