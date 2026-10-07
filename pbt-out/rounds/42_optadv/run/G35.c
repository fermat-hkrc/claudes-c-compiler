static int f1(int *p, int *q) { *p += 22; return *q * 2; }
static int f2(int *p, int *q) { *p += 566; return *q * 2; }
static int f3(int *p, int *q) { *p += 28; return *q * 2; }
static int f4(int *p, int *q) { *p += 285; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 188u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += f1(&cells[4], &cells[5]);
    arr[5 % 6] = 492; cells[0] += arr[5 % 6];
    sum += f2(&cells[0], &cells[0]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f3(&cells[7], &cells[3]);
    sum += f4(&cells[0], &cells[0]);
    arr[7 % 6] = 180; cells[1] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }