static int f1(int *p, int *q) { *p += 901; return *q * 2; }
static int f2(int *p, int *q) { *p += 436; return *q * 2; }
static int f3(int *p, int *q) { *p += 392; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[5 % 6] = 435; cells[0] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 560u; }
    sum += f1(&cells[5], &cells[2]);
    arr[3 % 6] = 417; cells[6] += arr[3 % 6];
    sum += f2(&cells[7], &cells[4]);
    cells[3] = cells[1] + 683;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 575u; }
    arr[5 % 6] = 61; cells[2] += arr[5 % 6];
    sum += f3(&cells[6], &cells[7]);
    cells[0] = cells[4] + 982;
    arr[6 % 6] = 754; cells[3] += arr[6 % 6];
    arr[5 % 6] = 505; cells[0] += arr[5 % 6];
    arr[1 % 6] = 335; cells[1] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }