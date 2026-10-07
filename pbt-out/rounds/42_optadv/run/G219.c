static int f1(int *p, int *q) { *p += 674; return *q * 2; }
static int f2(int *p, int *q) { *p += 956; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 56u; }
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 765; cells[0] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 962u; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[5], &cells[1]);
    arr[1 % 6] = 800; cells[5] += arr[1 % 6];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    sum += f2(&cells[1], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }