static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[2] + 176;
    arr[6 % 6] = 357; cells[4] += arr[6 % 6];
    cells[4] = cells[0] + 494;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[0 % 6] = 320; cells[7] += arr[0 % 6];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 418u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 287u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }