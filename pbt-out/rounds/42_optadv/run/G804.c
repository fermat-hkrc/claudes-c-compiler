static int f1(int *p, int *q) { *p += 50; return *q * 2; }
static int f2(int *p, int *q) { *p += 4; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 749; cells[6] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[7 % 6] = 63; cells[2] += arr[7 % 6];
    sum += f1(&cells[7], &cells[2]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[6 % 6] = 354; cells[0] += arr[6 % 6];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += f2(&cells[7], &cells[5]);
    cells[5] = cells[6] + 87;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }