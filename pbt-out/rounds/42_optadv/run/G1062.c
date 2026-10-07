static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[4] + 679;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[3] = cells[2] + 13;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 492u; }
    arr[5 % 6] = 237; cells[6] += arr[5 % 6];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 148u; }
    arr[1 % 6] = 279; cells[7] += arr[1 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f1(int *p, int *q) { *p += 906; return *q * 2; }
    sum += f1(&cells[7], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }