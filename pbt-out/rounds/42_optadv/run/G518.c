static int f1(int *p, int *q) { *p += 63; return *q * 2; }
static int f2(int *p, int *q) { *p += 441; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[4] = cells[5] + 873;
    sum += f1(&cells[1], &cells[7]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 754u; }
    arr[5 % 6] = 75; cells[6] += arr[5 % 6];
    cells[4] = cells[4] + 200;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f2(&cells[7], &cells[2]);
    cells[5] = cells[2] + 630;
    cells[2] = cells[3] + 647;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 7u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[0] = cells[6] + 599;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 391u; }
    cells[1] = cells[4] + 198;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }