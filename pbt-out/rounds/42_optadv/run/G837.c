static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 146u; }
    arr[6 % 6] = 288; cells[2] += arr[6 % 6];
    arr[1 % 6] = 744; cells[0] += arr[1 % 6];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 787u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 954u; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    arr[0 % 6] = 999; cells[7] += arr[0 % 6];
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[3 % 6] = 79; cells[6] += arr[3 % 6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[7] = cells[4] + 520;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }