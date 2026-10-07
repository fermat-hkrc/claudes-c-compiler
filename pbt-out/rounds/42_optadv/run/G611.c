static int f1(int *p, int *q) { *p += 644; return *q * 2; }
static int f2(int *p, int *q) { *p += 871; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[3], &cells[1]);
    cells[5] = cells[3] + 282;
    cells[0] = cells[2] + 907;
    cells[4] = cells[1] + 841;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 945u; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[0] = cells[2] + 664;
    arr[6 % 6] = 228; cells[4] += arr[6 % 6];
    sum += f2(&cells[2], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }