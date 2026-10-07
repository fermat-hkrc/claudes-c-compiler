static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 793u; }
    arr[2 % 6] = 89; cells[2] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 307; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    cells[2] = cells[0] + 294;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    static int f2(int *p, int *q) { *p += 123; return *q * 2; }
    sum += f2(&cells[0], &cells[3]);
    arr[3 % 6] = 319; cells[5] += arr[3 % 6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[2] = cells[2] + 199;
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 105u; }
    cells[4] = cells[0] + 178;
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 193u; }
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }