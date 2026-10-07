static int f1(int *p, int *q) { *p += 130; return *q * 2; }
static int f2(int *p, int *q) { *p += 914; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 243u; }
    arr[7 % 6] = 385; cells[1] += arr[7 % 6];
    sum += f1(&cells[1], &cells[3]);
    arr[1 % 6] = 237; cells[7] += arr[1 % 6];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 136u; }
    sum += f2(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[0] = cells[1] + 269;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 682u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }