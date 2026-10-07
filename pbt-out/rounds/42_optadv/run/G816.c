static int f1(int *p, int *q) { *p += 216; return *q * 2; }
static int f2(int *p, int *q) { *p += 281; return *q * 2; }
static int f3(int *p, int *q) { *p += 734; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += f1(&cells[1], &cells[5]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 586u; }
    sum += f2(&cells[2], &cells[6]);
    arr[6 % 6] = 321; cells[0] += arr[6 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 599u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f3(&cells[7], &cells[3]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 207u; }
    arr[3 % 6] = 952; cells[6] += arr[3 % 6];
    cells[6] = cells[1] + 474;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }