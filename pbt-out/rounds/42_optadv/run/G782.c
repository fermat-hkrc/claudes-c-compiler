static int f1(int *p, int *q) { *p += 647; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[3]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 777u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[1] = cells[5] + 601;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }