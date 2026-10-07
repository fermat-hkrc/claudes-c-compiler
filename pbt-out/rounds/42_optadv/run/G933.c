static int f1(int *p, int *q) { *p += 155; return *q * 2; }
static int f2(int *p, int *q) { *p += 433; return *q * 2; }
static int f3(int *p, int *q) { *p += 645; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[4] + 245;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 905u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[5 % 6] = 356; cells[5] += arr[5 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 3u; }
    sum += f1(&cells[2], &cells[6]);
    arr[4 % 6] = 243; cells[5] += arr[4 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 854u; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f2(&cells[5], &cells[4]);
    arr[3 % 6] = 643; cells[4] += arr[3 % 6];
    sum += f3(&cells[1], &cells[1]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }