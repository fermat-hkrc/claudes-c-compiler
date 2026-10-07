static int f1(int *p, int *q) { *p += 745; return *q * 2; }
static int f2(int *p, int *q) { *p += 161; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[5] + 574;
    sum += f2(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 769u; }
    cells[7] = cells[2] + 191;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }