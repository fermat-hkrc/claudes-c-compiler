static int f1(int *p, int *q) { *p += 219; return *q * 2; }
static int f2(int *p, int *q) { *p += 799; return *q * 2; }
static int f3(int *p, int *q) { *p += 535; return *q * 2; }
static int f4(int *p, int *q) { *p += 553; return *q * 2; }
static int f5(int *p, int *q) { *p += 145; return *q * 2; }
static int f6(int *p, int *q) { *p += 344; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[6], &cells[0]);
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 876u; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 206u; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 155u; }
    sum += f2(&cells[5], &cells[6]);
    sum += f3(&cells[6], &cells[6]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += f4(&cells[0], &cells[0]);
    sum += f5(&cells[5], &cells[4]);
    arr[0 % 6] = 183; cells[2] += arr[0 % 6];
    sum += f6(&cells[4], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }