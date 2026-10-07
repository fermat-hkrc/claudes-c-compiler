static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 143u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[0] + 501;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[0 % 6] = 780; cells[1] += arr[0 % 6];
    arr[7 % 6] = 430; cells[6] += arr[7 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f1(int *p, int *q) { *p += 442; return *q * 2; }
    sum += f1(&cells[7], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[6] + 648;
    arr[0 % 6] = 995; cells[7] += arr[0 % 6];
    arr[0 % 6] = 341; cells[7] += arr[0 % 6];
    arr[7 % 6] = 593; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 457u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 152u; }
    cells[7] = cells[0] + 333;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }