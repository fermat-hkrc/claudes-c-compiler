static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[0 % 6] = 633; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 320u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 764u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    static int f1(int *p, int *q) { *p += 50; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[6 % 6] = 199; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f2(int *p, int *q) { *p += 673; return *q * 2; }
    sum += f2(&cells[7], &cells[3]);
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 54u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[0 % 6] = 969; cells[3] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }