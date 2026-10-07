static int f1(int *p, int *q) { *p += 317; return *q * 2; }
static int f2(int *p, int *q) { *p += 390; return *q * 2; }
static int f3(int *p, int *q) { *p += 874; return *q * 2; }
static int f4(int *p, int *q) { *p += 88; return *q * 2; }
static int f5(int *p, int *q) { *p += 21; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 731u; }
    sum += f1(&cells[3], &cells[3]);
    sum += f2(&cells[0], &cells[4]);
    cells[6] = cells[4] + 719;
    sum += f3(&cells[5], &cells[0]);
    cells[4] = cells[5] + 449;
    arr[1 % 6] = 228; cells[4] += arr[1 % 6];
    sum += f4(&cells[3], &cells[1]);
    cells[0] = cells[1] + 745;
    sum += f5(&cells[6], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }