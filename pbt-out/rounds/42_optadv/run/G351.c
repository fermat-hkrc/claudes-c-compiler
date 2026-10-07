static int f1(int *p, int *q) { *p += 168; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 158u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 576u; }
    cells[5] = cells[0] + 776;
    arr[7 % 6] = 820; cells[2] += arr[7 % 6];
    sum += f1(&cells[1], &cells[3]);
    cells[5] = cells[3] + 591;
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 69u; }
    arr[6 % 6] = 828; cells[4] += arr[6 % 6];
    arr[6 % 6] = 159; cells[7] += arr[6 % 6];
    arr[5 % 6] = 730; cells[2] += arr[5 % 6];
    cells[1] = cells[0] + 150;
    cells[5] = cells[4] + 174;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }