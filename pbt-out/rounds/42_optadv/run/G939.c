static int f1(int *p, int *q) { *p += 977; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[7] + 366;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[1 % 6] = 625; cells[7] += arr[1 % 6];
    cells[5] = cells[4] + 49;
    arr[4 % 6] = 574; cells[0] += arr[4 % 6];
    arr[5 % 6] = 261; cells[3] += arr[5 % 6];
    cells[0] = cells[0] + 473;
    sum += f1(&cells[1], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }