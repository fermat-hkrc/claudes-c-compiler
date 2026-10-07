static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 214; cells[3] += arr[7 % 6];
    arr[6 % 6] = 369; cells[5] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[2] + 71;
    arr[3 % 6] = 552; cells[2] += arr[3 % 6];
    cells[1] = cells[4] + 962;
    arr[2 % 6] = 995; cells[1] += arr[2 % 6];
    cells[6] = cells[3] + 179;
    cells[7] = cells[7] + 516;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 552u; }
    cells[2] = cells[5] + 952;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }