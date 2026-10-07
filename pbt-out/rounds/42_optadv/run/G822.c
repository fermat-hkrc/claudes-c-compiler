static int f1(int *p, int *q) { *p += 24; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 574u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 660u; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[6] = cells[1] + 97;
    cells[0] = cells[7] + 881;
    sum += f1(&cells[3], &cells[3]);
    arr[0 % 6] = 777; cells[7] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 218u; }
    cells[7] = cells[1] + 338;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }