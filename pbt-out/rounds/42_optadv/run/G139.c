static int f1(int *p, int *q) { *p += 64; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[5 % 6] = 611; cells[4] += arr[5 % 6];
    sum += f1(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[5] = cells[7] + 750;
    cells[1] = cells[5] + 552;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 111u; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 339u; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 370u; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 741u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[0 % 6] = 160; cells[2] += arr[0 % 6];
    cells[5] = cells[7] + 514;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 588u; }
    arr[0 % 6] = 855; cells[6] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }