static int f1(int *p, int *q) { *p += 98; return *q * 2; }
static int f2(int *p, int *q) { *p += 636; return *q * 2; }
static int f3(int *p, int *q) { *p += 426; return *q * 2; }
static int f4(int *p, int *q) { *p += 689; return *q * 2; }
static int f5(int *p, int *q) { *p += 374; return *q * 2; }
static int f6(int *p, int *q) { *p += 879; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 980u; }
    sum += f1(&cells[2], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 193u; }
    sum += f2(&cells[6], &cells[4]);
    arr[0 % 6] = 472; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 49u; }
    arr[4 % 6] = 506; cells[0] += arr[4 % 6];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 808u; }
    sum += f3(&cells[0], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 836u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f4(&cells[4], &cells[1]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += f5(&cells[0], &cells[1]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 327u; }
    sum += f6(&cells[7], &cells[4]);
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 97u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }