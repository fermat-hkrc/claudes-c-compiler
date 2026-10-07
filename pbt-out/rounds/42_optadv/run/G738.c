static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    static int f1(int *p, int *q) { *p += 831; return *q * 2; }
    sum += f1(&cells[0], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 271u; }
    cells[6] = cells[5] + 990;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[3] = cells[5] + 125;
    arr[4 % 6] = 647; cells[4] += arr[4 % 6];
    static int f2(int *p, int *q) { *p += 730; return *q * 2; }
    sum += f2(&cells[2], &cells[3]);
    cells[7] = cells[1] + 939;
    arr[4 % 6] = 313; cells[7] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    cells[5] = cells[4] + 610;
    static int f3(int *p, int *q) { *p += 374; return *q * 2; }
    sum += f3(&cells[5], &cells[7]);
    arr[4 % 6] = 319; cells[2] += arr[4 % 6];
    static int f4(int *p, int *q) { *p += 297; return *q * 2; }
    sum += f4(&cells[5], &cells[6]);
    arr[4 % 6] = 677; cells[2] += arr[4 % 6];
    arr[5 % 6] = 442; cells[0] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }