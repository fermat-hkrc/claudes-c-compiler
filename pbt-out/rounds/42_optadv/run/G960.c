static int f1(int *p, int *q) { *p += 774; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[6] + 922;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[4 % 6] = 541; cells[7] += arr[4 % 6];
    arr[5 % 6] = 16; cells[7] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 576u; }
    arr[2 % 6] = 303; cells[6] += arr[2 % 6];
    cells[3] = cells[3] + 107;
    sum += f1(&cells[2], &cells[2]);
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 429u; }
    cells[2] = cells[4] + 216;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }