static int f1(int *p, int *q) { *p += 26; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 374u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 343u; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 481u; }
    cells[1] = cells[7] + 228;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 153u; }
    sum += f1(&cells[3], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[2] + 315;
    cells[0] = cells[6] + 516;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 711u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[1] = cells[3] + 3;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 470u; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 34u; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }