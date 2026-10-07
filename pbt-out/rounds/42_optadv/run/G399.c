static int f1(int *p, int *q) { *p += 111; return *q * 2; }
static int f2(int *p, int *q) { *p += 103; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    cells[4] = cells[0] + 801;
    sum += f1(&cells[7], &cells[7]);
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 605u; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 959u; }
    sum += f2(&cells[2], &cells[0]);
    arr[7 % 6] = 560; cells[4] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }