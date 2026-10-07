static int f1(int *p, int *q) { *p += 493; return *q * 2; }
static int f2(int *p, int *q) { *p += 204; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 628; cells[4] += arr[5 % 6];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 12u; }
    cells[3] = cells[7] + 980;
    arr[7 % 6] = 938; cells[3] += arr[7 % 6];
    arr[7 % 6] = 417; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[4], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[0] + 911;
    sum += f2(&cells[7], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }