static int f1(int *p, int *q) { *p += 449; return *q * 2; }
static int f2(int *p, int *q) { *p += 575; return *q * 2; }
static int f3(int *p, int *q) { *p += 782; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 424; cells[4] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 941u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 207u; }
    cells[4] = cells[0] + 858;
    cells[0] = cells[1] + 462;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[4 % 6] = 955; cells[1] += arr[4 % 6];
    arr[4 % 6] = 885; cells[5] += arr[4 % 6];
    sum += f1(&cells[3], &cells[2]);
    cells[3] = cells[0] + 986;
    cells[4] = cells[0] + 622;
    cells[0] = cells[3] + 156;
    cells[0] = cells[2] + 476;
    arr[7 % 6] = 750; cells[4] += arr[7 % 6];
    cells[6] = cells[4] + 123;
    sum += f2(&cells[1], &cells[1]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += f3(&cells[2], &cells[1]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }