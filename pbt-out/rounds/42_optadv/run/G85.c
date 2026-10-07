static int f1(int *p, int *q) { *p += 602; return *q * 2; }
static int f2(int *p, int *q) { *p += 545; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 659u; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 759; cells[6] += arr[3 % 6];
    arr[1 % 6] = 297; cells[2] += arr[1 % 6];
    cells[5] = cells[2] + 191;
    cells[4] = cells[3] + 840;
    arr[7 % 6] = 441; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[1], &cells[6]);
    sum += f2(&cells[6], &cells[3]);
    arr[6 % 6] = 328; cells[3] += arr[6 % 6];
    cells[4] = cells[7] + 428;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }