static int f1(int *p, int *q) { *p += 659; return *q * 2; }
static int f2(int *p, int *q) { *p += 474; return *q * 2; }
static int f3(int *p, int *q) { *p += 296; return *q * 2; }
static int f4(int *p, int *q) { *p += 443; return *q * 2; }
static int f5(int *p, int *q) { *p += 983; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 31u; }
    arr[5 % 6] = 479; cells[4] += arr[5 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 88u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 782u; }
    arr[7 % 6] = 174; cells[3] += arr[7 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 543u; }
    arr[1 % 6] = 337; cells[2] += arr[1 % 6];
    cells[6] = cells[1] + 866;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[3], &cells[4]);
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 408u; }
    cells[1] = cells[0] + 402;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 436u; }
    sum += f2(&cells[1], &cells[3]);
    sum += f3(&cells[4], &cells[1]);
    sum += f4(&cells[1], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f5(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }