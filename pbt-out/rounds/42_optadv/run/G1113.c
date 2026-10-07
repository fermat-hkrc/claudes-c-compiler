static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[1] + 215;
    cells[3] = cells[0] + 688;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    static int f1(int *p, int *q) { *p += 910; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 665u; }
    arr[3 % 6] = 456; cells[3] += arr[3 % 6];
    arr[7 % 6] = 668; cells[7] += arr[7 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 133u; }
    cells[7] = cells[1] + 547;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 349u; }
    arr[5 % 6] = 278; cells[5] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 859u; }
    static int f2(int *p, int *q) { *p += 354; return *q * 2; }
    sum += f2(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }