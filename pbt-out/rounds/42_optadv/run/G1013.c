static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 704; cells[5] += arr[5 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 56u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[4] = cells[3] + 753;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 190u; }
    cells[7] = cells[1] + 840;
    arr[7 % 6] = 390; cells[4] += arr[7 % 6];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    cells[3] = cells[0] + 39;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[3 % 6] = 129; cells[7] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 141u; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 802u; }
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }