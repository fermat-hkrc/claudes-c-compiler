static int f1(int *p, int *q) { *p += 444; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[5] = cells[4] + 543;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 164u; }
    arr[6 % 6] = 736; cells[4] += arr[6 % 6];
    arr[0 % 6] = 475; cells[1] += arr[0 % 6];
    cells[6] = cells[7] + 848;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[0] + 204;
    cells[3] = cells[2] + 168;
    cells[6] = cells[0] + 315;
    sum += f1(&cells[4], &cells[7]);
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 503u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }