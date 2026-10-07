static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[2 % 6] = 768; cells[0] += arr[2 % 6];
    cells[6] = cells[1] + 940;
    cells[1] = cells[5] + 456;
    cells[4] = cells[0] + 959;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 582u; }
    cells[6] = cells[2] + 740;
    arr[7 % 6] = 367; cells[0] += arr[7 % 6];
    cells[7] = cells[2] + 964;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }