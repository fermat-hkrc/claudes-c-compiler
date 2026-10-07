static int f1(int *p, int *q) { *p += 50; return *q * 2; }
static int f2(int *p, int *q) { *p += 778; return *q * 2; }
static int f3(int *p, int *q) { *p += 157; return *q * 2; }
static int f4(int *p, int *q) { *p += 79; return *q * 2; }
static int f5(int *p, int *q) { *p += 695; return *q * 2; }
static int f6(int *p, int *q) { *p += 43; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[3] + 782;
    sum += f1(&cells[7], &cells[0]);
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f2(&cells[3], &cells[5]);
    sum += f3(&cells[6], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[4] = cells[5] + 446;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += f4(&cells[6], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f5(&cells[3], &cells[4]);
    cells[7] = cells[4] + 690;
    cells[0] = cells[5] + 67;
    sum += f6(&cells[4], &cells[4]);
    cells[3] = cells[3] + 938;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }