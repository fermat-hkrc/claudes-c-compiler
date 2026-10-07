static int f1(int *p, int *q) { *p += 623; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[0 % 6] = 453; cells[5] += arr[0 % 6];
    arr[6 % 6] = 98; cells[4] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[5] = cells[7] + 228;
    sum += f1(&cells[5], &cells[0]);
    arr[7 % 6] = 237; cells[0] += arr[7 % 6];
    cells[7] = cells[3] + 126;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[6] = cells[1] + 469;
    cells[3] = cells[1] + 442;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    arr[0 % 6] = 782; cells[2] += arr[0 % 6];
    cells[2] = cells[2] + 976;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }