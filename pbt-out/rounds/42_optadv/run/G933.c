static int f1(int *p, int *q) { *p += 32; return *q * 2; }
static int f2(int *p, int *q) { *p += 701; return *q * 2; }
static int f3(int *p, int *q) { *p += 68; return *q * 2; }
static int f4(int *p, int *q) { *p += 263; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[3]);
    sum += f2(&cells[2], &cells[4]);
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    cells[6] = cells[3] + 581;
    sum += f3(&cells[6], &cells[2]);
    sum += f4(&cells[2], &cells[5]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 918u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }