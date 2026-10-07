static int f1(int *p, int *q) { *p += 317; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[7 % 6] = 450; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 109u; }
    sum += f1(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[0 % 6] = 363; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[2] + 348;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    arr[6 % 6] = 743; cells[6] += arr[6 % 6];
    arr[0 % 6] = 108; cells[7] += arr[0 % 6];
    arr[4 % 6] = 916; cells[5] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }