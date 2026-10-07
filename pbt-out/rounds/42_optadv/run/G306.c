static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 624u; }
    cells[1] = cells[3] + 725;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[0 % 6] = 823; cells[0] += arr[0 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 516u; }
    cells[2] = cells[3] + 207;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 300u; }
    static int f1(int *p, int *q) { *p += 506; return *q * 2; }
    sum += f1(&cells[3], &cells[6]);
    static int f2(int *p, int *q) { *p += 457; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 230; cells[0] += arr[1 % 6];
    static int f3(int *p, int *q) { *p += 443; return *q * 2; }
    sum += f3(&cells[2], &cells[5]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 592u; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    arr[0 % 6] = 157; cells[1] += arr[0 % 6];
    static int f4(int *p, int *q) { *p += 461; return *q * 2; }
    sum += f4(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 885; cells[0] += arr[2 % 6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }