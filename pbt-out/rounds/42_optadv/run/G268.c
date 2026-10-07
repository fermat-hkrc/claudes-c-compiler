static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 876; cells[0] += arr[5 % 6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 414u; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 441u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[5] = cells[1] + 592;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }