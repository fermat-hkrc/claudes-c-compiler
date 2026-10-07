static int f1(int *p, int *q) { *p += 224; return *q * 2; }
static int f2(int *p, int *q) { *p += 631; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[0] + 406;
    sum += f1(&cells[4], &cells[6]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 710u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 939u; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 840u; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 408u; }
    cells[1] = cells[6] + 410;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 511u; }
    arr[7 % 6] = 682; cells[1] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f2(&cells[5], &cells[2]);
    arr[6 % 6] = 616; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }