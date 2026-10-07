static int f1(int *p, int *q) { *p += 849; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 638u; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[1] = cells[2] + 955;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[0 % 6] = 894; cells[1] += arr[0 % 6];
    cells[5] = cells[7] + 911;
    arr[7 % 6] = 556; cells[2] += arr[7 % 6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    arr[1 % 6] = 700; cells[0] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[5] = cells[0] + 416;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }