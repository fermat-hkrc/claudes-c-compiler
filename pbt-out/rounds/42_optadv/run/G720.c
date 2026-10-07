static int f1(int *p, int *q) { *p += 99; return *q * 2; }
static int f2(int *p, int *q) { *p += 834; return *q * 2; }
static int f3(int *p, int *q) { *p += 72; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[3] = cells[4] + 338;
    sum += f1(&cells[4], &cells[0]);
    sum += f2(&cells[2], &cells[6]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 555u; }
    sum += f3(&cells[0], &cells[7]);
    arr[2 % 6] = 648; cells[1] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[2] = cells[5] + 342;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 47u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[6] = cells[0] + 920;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }