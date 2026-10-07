static int f1(int *p, int *q) { *p += 764; return *q * 2; }
static int f2(int *p, int *q) { *p += 289; return *q * 2; }
static int f3(int *p, int *q) { *p += 610; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[3] + 356;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[5], &cells[7]);
    cells[3] = cells[7] + 318;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 383u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[7] + 427;
    cells[0] = cells[7] + 203;
    sum += f2(&cells[6], &cells[5]);
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    cells[1] = cells[7] + 742;
    arr[6 % 6] = 853; cells[2] += arr[6 % 6];
    arr[3 % 6] = 923; cells[3] += arr[3 % 6];
    sum += f3(&cells[0], &cells[6]);
    cells[4] = cells[1] + 356;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }