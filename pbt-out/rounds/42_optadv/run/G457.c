static int f1(int *p, int *q) { *p += 942; return *q * 2; }
static int f2(int *p, int *q) { *p += 600; return *q * 2; }
static int f3(int *p, int *q) { *p += 387; return *q * 2; }
static int f4(int *p, int *q) { *p += 676; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 531; cells[6] += arr[2 % 6];
    sum += f1(&cells[7], &cells[4]);
    sum += f2(&cells[6], &cells[2]);
    sum += f3(&cells[5], &cells[5]);
    cells[6] = cells[2] + 364;
    sum += f4(&cells[5], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }