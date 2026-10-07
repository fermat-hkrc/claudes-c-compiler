static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 726; cells[2] += arr[1 % 6];
    cells[5] = cells[3] + 944;
    arr[5 % 6] = 666; cells[4] += arr[5 % 6];
    cells[5] = cells[5] + 861;
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 55u; }
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[5] = cells[2] + 962;
    arr[4 % 6] = 306; cells[7] += arr[4 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 385u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 942u; }
    static int f1(int *p, int *q) { *p += 632; return *q * 2; }
    sum += f1(&cells[7], &cells[3]);
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 179u; }
    cells[7] = cells[4] + 266;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 985u; }
    arr[7 % 6] = 56; cells[6] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }