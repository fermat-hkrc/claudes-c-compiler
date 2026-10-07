static int f1(int *p, int *q) { *p += 202; return *q * 2; }
static int f2(int *p, int *q) { *p += 910; return *q * 2; }
static int f3(int *p, int *q) { *p += 174; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 293u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[5 % 6] = 990; cells[7] += arr[5 % 6];
    arr[2 % 6] = 273; cells[3] += arr[2 % 6];
    arr[2 % 6] = 545; cells[4] += arr[2 % 6];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 214u; }
    cells[3] = cells[4] + 782;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 151u; }
    cells[1] = cells[5] + 752;
    sum += f1(&cells[5], &cells[2]);
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f2(&cells[6], &cells[5]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 175u; }
    sum += f3(&cells[4], &cells[5]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 894u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }