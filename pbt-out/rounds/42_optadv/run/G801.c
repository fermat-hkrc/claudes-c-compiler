static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    static int f1(int *p, int *q) { *p += 926; return *q * 2; }
    sum += f1(&cells[7], &cells[7]);
    static int f2(int *p, int *q) { *p += 454; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 792u; }
    static int f3(int *p, int *q) { *p += 843; return *q * 2; }
    sum += f3(&cells[2], &cells[2]);
    static int f4(int *p, int *q) { *p += 516; return *q * 2; }
    sum += f4(&cells[2], &cells[4]);
    arr[3 % 6] = 417; cells[1] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    static int f5(int *p, int *q) { *p += 962; return *q * 2; }
    sum += f5(&cells[5], &cells[6]);
    cells[2] = cells[5] + 335;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 134u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 159u; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 373u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }