static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 4; cells[1] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    arr[0 % 6] = 960; cells[3] += arr[0 % 6];
    arr[3 % 6] = 10; cells[3] += arr[3 % 6];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 474u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }