static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[7] + 665;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[7 % 6] = 297; cells[2] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 659; return *q * 2; }
    sum += f1(&cells[7], &cells[0]);
    cells[4] = cells[5] + 363;
    static int f2(int *p, int *q) { *p += 668; return *q * 2; }
    sum += f2(&cells[2], &cells[5]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    arr[7 % 6] = 380; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    arr[3 % 6] = 241; cells[1] += arr[3 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 846u; }
    static int f3(int *p, int *q) { *p += 702; return *q * 2; }
    sum += f3(&cells[4], &cells[1]);
    arr[1 % 6] = 219; cells[4] += arr[1 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 170u; }
    cells[2] = cells[5] + 748;
    arr[7 % 6] = 460; cells[1] += arr[7 % 6];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    static int f4(int *p, int *q) { *p += 768; return *q * 2; }
    sum += f4(&cells[2], &cells[7]);
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }