static int f1(int *p, int *q) { *p += 585; return *q * 2; }
static int f2(int *p, int *q) { *p += 221; return *q * 2; }
static int f3(int *p, int *q) { *p += 815; return *q * 2; }
static int f4(int *p, int *q) { *p += 501; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    sum += f1(&cells[5], &cells[2]);
    sum += f2(&cells[7], &cells[4]);
    arr[7 % 6] = 85; cells[4] += arr[7 % 6];
    cells[2] = cells[6] + 942;
    cells[3] = cells[2] + 655;
    sum += f3(&cells[1], &cells[1]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += f4(&cells[1], &cells[3]);
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 996u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }