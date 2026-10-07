static int f1(int *p, int *q) { *p += 471; return *q * 2; }
static int f2(int *p, int *q) { *p += 778; return *q * 2; }
static int f3(int *p, int *q) { *p += 785; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 920u; }
    sum += f1(&cells[7], &cells[7]);
    cells[4] = cells[7] + 517;
    arr[7 % 6] = 177; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[4] = cells[5] + 960;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[6] = cells[0] + 324;
    sum += f2(&cells[5], &cells[5]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 514u; }
    cells[7] = cells[0] + 566;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 507u; }
    arr[7 % 6] = 305; cells[2] += arr[7 % 6];
    sum += f3(&cells[1], &cells[7]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }