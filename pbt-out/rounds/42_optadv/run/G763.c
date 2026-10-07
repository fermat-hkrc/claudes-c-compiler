static int f1(int *p, int *q) { *p += 684; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[5] = cells[7] + 192;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[3] + 756;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 215u; }
    arr[3 % 6] = 944; cells[2] += arr[3 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[5 % 6] = 930; cells[7] += arr[5 % 6];
    cells[6] = cells[7] + 913;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 746u; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 867u; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 573u; }
    sum += f1(&cells[5], &cells[1]);
    arr[7 % 6] = 830; cells[1] += arr[7 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 555u; }
    arr[2 % 6] = 389; cells[4] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 588u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }