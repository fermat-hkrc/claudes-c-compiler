static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 400; cells[5] += arr[3 % 6];
    cells[1] = cells[5] + 618;
    arr[4 % 6] = 118; cells[3] += arr[4 % 6];
    cells[4] = cells[6] + 389;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 471u; }
    cells[2] = cells[2] + 967;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[7] = cells[7] + 128;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }