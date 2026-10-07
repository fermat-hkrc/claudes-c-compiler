static int f1(int *p, int *q) { *p += 874; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 975u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[6], &cells[3]);
    arr[5 % 6] = 724; cells[6] += arr[5 % 6];
    cells[0] = cells[1] + 75;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 65u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }