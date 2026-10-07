static int f1(int *p, int *q) { *p += 311; return *q * 2; }
static int f2(int *p, int *q) { *p += 647; return *q * 2; }
static int f3(int *p, int *q) { *p += 5; return *q * 2; }
static int f4(int *p, int *q) { *p += 242; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[1]);
    sum += f2(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f3(&cells[0], &cells[4]);
    cells[3] = cells[6] + 928;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 914u; }
    sum += f4(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }