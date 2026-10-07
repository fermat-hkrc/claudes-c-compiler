static int f1(int *p, int *q) { *p += 98; return *q * 2; }
static int f2(int *p, int *q) { *p += 216; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[1] + 872;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    arr[3 % 6] = 668; cells[3] += arr[3 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 321u; }
    sum += f1(&cells[1], &cells[3]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 943u; }
    arr[6 % 6] = 20; cells[7] += arr[6 % 6];
    arr[7 % 6] = 744; cells[0] += arr[7 % 6];
    sum += f2(&cells[4], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }