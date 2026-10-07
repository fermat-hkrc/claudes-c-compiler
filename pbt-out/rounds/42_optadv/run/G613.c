static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[4] + 728;
    cells[4] = cells[4] + 536;
    arr[4 % 6] = 242; cells[4] += arr[4 % 6];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 662u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 280u; }
    cells[7] = cells[5] + 31;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }