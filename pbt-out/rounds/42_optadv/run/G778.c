static int f1(int *p, int *q) { *p += 176; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 173u; }
    sum += f1(&cells[7], &cells[3]);
    arr[6 % 6] = 966; cells[5] += arr[6 % 6];
    cells[6] = cells[3] + 183;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }