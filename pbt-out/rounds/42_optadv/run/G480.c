static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 888; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    arr[6 % 6] = 655; cells[5] += arr[6 % 6];
    cells[5] = cells[2] + 136;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 206u; }
    arr[7 % 6] = 241; cells[0] += arr[7 % 6];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    cells[7] = cells[5] + 20;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 280u; }
    static int f2(int *p, int *q) { *p += 682; return *q * 2; }
    sum += f2(&cells[6], &cells[7]);
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 67u; }
    arr[1 % 6] = 828; cells[0] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }