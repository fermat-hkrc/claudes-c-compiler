static int f1(int *p, int *q) { *p += 462; return *q * 2; }
static int f2(int *p, int *q) { *p += 816; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 478; cells[4] += arr[0 % 6];
    arr[7 % 6] = 86; cells[3] += arr[7 % 6];
    cells[1] = cells[7] + 769;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 659u; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += f1(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[7] = cells[7] + 148;
    sum += f2(&cells[7], &cells[4]);
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 443u; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }