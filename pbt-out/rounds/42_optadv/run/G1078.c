static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[7] + 431;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 858; cells[2] += arr[4 % 6];
    cells[6] = cells[0] + 925;
    arr[6 % 6] = 322; cells[6] += arr[6 % 6];
    arr[5 % 6] = 944; cells[2] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 271u; }
    cells[1] = cells[2] + 382;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 192u; }
    arr[2 % 6] = 912; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[3] = cells[2] + 875;
    arr[4 % 6] = 309; cells[3] += arr[4 % 6];
    cells[5] = cells[0] + 345;
    cells[4] = cells[6] + 222;
    cells[0] = cells[3] + 204;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    static int f1(int *p, int *q) { *p += 625; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 814u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }