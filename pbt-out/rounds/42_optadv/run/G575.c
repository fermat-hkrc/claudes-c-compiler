static int f1(int *p, int *q) { *p += 967; return *q * 2; }
static int f2(int *p, int *q) { *p += 871; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[6] + 982;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 322u; }
    sum += f1(&cells[2], &cells[0]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[4 % 6] = 832; cells[7] += arr[4 % 6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    sum += f2(&cells[6], &cells[1]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }