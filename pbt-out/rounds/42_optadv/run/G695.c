static int f1(int *p, int *q) { *p += 100; return *q * 2; }
static int f2(int *p, int *q) { *p += 531; return *q * 2; }
static int f3(int *p, int *q) { *p += 790; return *q * 2; }
static int f4(int *p, int *q) { *p += 829; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 276u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 425u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 961u; }
    arr[7 % 6] = 850; cells[0] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[4], &cells[7]);
    sum += f3(&cells[2], &cells[3]);
    cells[6] = cells[4] + 869;
    arr[2 % 6] = 342; cells[4] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 608u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 283u; }
    sum += f4(&cells[6], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[3] = cells[3] + 424;
    arr[2 % 6] = 741; cells[0] += arr[2 % 6];
    arr[5 % 6] = 728; cells[3] += arr[5 % 6];
    cells[6] = cells[7] + 126;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }