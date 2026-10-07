static int f1(int *p, int *q) { *p += 253; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    arr[5 % 6] = 59; cells[4] += arr[5 % 6];
    arr[3 % 6] = 575; cells[1] += arr[3 % 6];
    cells[0] = cells[7] + 320;
    sum += f1(&cells[5], &cells[7]);
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 444u; }
    arr[2 % 6] = 918; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }