static int f1(int *p, int *q) { *p += 920; return *q * 2; }
static int f2(int *p, int *q) { *p += 57; return *q * 2; }
static int f3(int *p, int *q) { *p += 300; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 552; cells[2] += arr[5 % 6];
    cells[2] = cells[3] + 582;
    sum += f1(&cells[1], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[0], &cells[3]);
    arr[7 % 6] = 127; cells[3] += arr[7 % 6];
    cells[7] = cells[5] + 76;
    sum += f3(&cells[4], &cells[3]);
    cells[6] = cells[1] + 39;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }