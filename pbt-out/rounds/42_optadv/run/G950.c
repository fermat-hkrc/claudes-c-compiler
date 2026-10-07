static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 480; cells[2] += arr[0 % 6];
    cells[0] = cells[0] + 465;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 126u; }
    cells[4] = cells[2] + 273;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }