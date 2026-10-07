static int f1(int *p, int *q) { *p += 201; return *q * 2; }
static int f2(int *p, int *q) { *p += 76; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 767; cells[5] += arr[5 % 6];
    cells[7] = cells[7] + 794;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += f1(&cells[2], &cells[0]);
    cells[3] = cells[5] + 343;
    arr[4 % 6] = 743; cells[1] += arr[4 % 6];
    arr[6 % 6] = 93; cells[6] += arr[6 % 6];
    sum += f2(&cells[0], &cells[1]);
    cells[5] = cells[2] + 38;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }