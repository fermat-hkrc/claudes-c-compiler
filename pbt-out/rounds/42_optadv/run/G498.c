static int f1(int *p, int *q) { *p += 981; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 235; cells[2] += arr[4 % 6];
    arr[2 % 6] = 538; cells[5] += arr[2 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[4 % 6] = 422; cells[0] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 710u; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 856u; }
    arr[3 % 6] = 171; cells[6] += arr[3 % 6];
    arr[5 % 6] = 175; cells[7] += arr[5 % 6];
    cells[5] = cells[1] + 145;
    sum += f1(&cells[1], &cells[3]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    arr[1 % 6] = 396; cells[1] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }