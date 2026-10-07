static int f1(int *p, int *q) { *p += 40; return *q * 2; }
static int f2(int *p, int *q) { *p += 666; return *q * 2; }
static int f3(int *p, int *q) { *p += 670; return *q * 2; }
static int f4(int *p, int *q) { *p += 271; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 631;
    arr[2 % 6] = 685; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[5] = cells[1] + 14;
    arr[6 % 6] = 370; cells[3] += arr[6 % 6];
    sum += f1(&cells[2], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += f2(&cells[3], &cells[1]);
    sum += f3(&cells[5], &cells[5]);
    cells[0] = cells[0] + 939;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 311u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[0] + 276;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 352u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f4(&cells[6], &cells[7]);
    arr[4 % 6] = 368; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 282; cells[1] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }