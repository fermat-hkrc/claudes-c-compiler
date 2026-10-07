static int f1(int *p, int *q) { *p += 231; return *q * 2; }
static int f2(int *p, int *q) { *p += 838; return *q * 2; }
static int f3(int *p, int *q) { *p += 696; return *q * 2; }
static int f4(int *p, int *q) { *p += 406; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[3]);
    sum += f2(&cells[1], &cells[6]);
    sum += f3(&cells[0], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[3] = cells[2] + 416;
    arr[3 % 6] = 990; cells[7] += arr[3 % 6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 444u; }
    cells[1] = cells[4] + 860;
    sum += f4(&cells[2], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }