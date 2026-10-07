static int f1(int *p, int *q) { *p += 873; return *q * 2; }
static int f2(int *p, int *q) { *p += 459; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 489u; }
    arr[6 % 6] = 533; cells[6] += arr[6 % 6];
    cells[7] = cells[1] + 976;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 757u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[4 % 6] = 329; cells[7] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 518u; }
    cells[2] = cells[0] + 970;
    cells[5] = cells[1] + 27;
    sum += f1(&cells[0], &cells[5]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += f2(&cells[6], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }